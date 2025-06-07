use bevy::math::IVec2;
#[allow(unused_imports)]
use tracing::warn;

use crate::model::{
    actor::ActorId, board::Board, direction::Dir, direction::RelDir, game::Game, program::Action,
};

#[derive(Debug, Clone)]
pub enum Cmd {
    Activate(ActorId),
    Deactivate(ActorId),
    MoveTo(Result<Dest, Dest>),
    TryPush(Dest),
    Turn(ActorId, RelDir),
    CompletePush(Dest),
    CancelPush(Dest),
    Done,
    Hit(Dest),
}

#[derive(Debug, Clone)]
pub struct Dest {
    pub from_actor_id: ActorId,
    pub to_coord: IVec2,
}

pub struct Runner {
    saved_board: Board,
    game: Game,
    commands: Vec<Cmd>,
    activated_actors: Vec<ActorId>,
}

impl Runner {
    pub fn new(game: Game) -> Self {
        Self {
            saved_board: game.board().clone(),
            game,
            commands: Vec::new(),
            activated_actors: Vec::new(),
        }
    }

    fn push_cmd(&mut self, cmd: Cmd) {
        self.commands.push(cmd);
    }

    fn push_activated(&mut self, actor_id: ActorId) {
        self.activated_actors.push(actor_id);
    }

    fn pop_activated(&mut self) -> Option<ActorId> {
        self.activated_actors.pop()
    }

    fn activate(&mut self, actor_id: ActorId) {
        let view = self.game.actor_view(&actor_id).unwrap();
        if !view.actor.activated && view.actor.activations_left > 0 {
            self.game.update_actor(&actor_id, |actor| {
                actor.activated = true;
                actor.activations_left -= 1;
            });
            self.push_activated(actor_id);
            self.push_cmd(Cmd::Activate(actor_id));
        }
    }

    fn deactivate(&mut self, actor_id: ActorId) {
        self.game.update_actor(&actor_id, |actor| {
            actor.activated = false;
        });
    }

    pub fn run(&mut self) -> (Game, Vec<Cmd>) {
        self.activate(self.game.board().start_actor_id());

        while let Some(actor_id) = self.pop_activated() {
            self.run_actor(actor_id);
        }

        // return the end_game
        let mut end_game = self.game.clone();
        end_game.set_board(self.saved_board.clone());
        // warn!("saved_board: {:?}", self.saved_board);
        self.push_cmd(Cmd::Done);

        (end_game, self.commands.drain(..).collect())
    }

    fn run_actor(&mut self, actor_id: ActorId) {
        let view = self.game.actor_view(&actor_id).unwrap();
        let program = &view.actor_type.program;
        for step in program.iter() {
            match step {
                Action::Forward => self.move_actor_forward(actor_id),
                Action::Push(rel_dir) => self.actor_push(actor_id, *rel_dir),
                Action::Turn(rel_dir) => self.turn_actor(actor_id, *rel_dir),
                Action::Hit(ivec2s) => self.process_actor_hits(actor_id, ivec2s),
            }
        }
        self.deactivate(actor_id);
        self.push_cmd(Cmd::Deactivate(actor_id));
    }

    fn move_actor_forward(&mut self, actor_id: ActorId) {
        let view = self.game.actor_view(&actor_id).unwrap();

        let coord = view.actor.coord;
        let new_coord = view.actor.looks_to.apply_to(coord);

        if self.game.board().coord_to_actor_id(&new_coord).is_some() {
            // warn!("no move {new_coord}");
            self.push_cmd(Cmd::MoveTo(Err(Dest {
                from_actor_id: actor_id,
                to_coord: new_coord,
            })));
        } else {
            // warn!("move to {new_coord}");
            self.game.update_actor(&actor_id, |actor| {
                actor.coord = new_coord;
            });
            self.push_cmd(Cmd::MoveTo(Ok(Dest {
                from_actor_id: actor_id,
                to_coord: new_coord,
            })));
        }
    }

    fn actor_line<'a>(
        &'a self,
        mut coord: IVec2,
        dir: Dir,
    ) -> impl Iterator<Item = (IVec2, Option<ActorId>)> + 'a {
        //let board = self.game.board().clone();
        std::iter::from_fn(move || {
            let result = Some((coord, self.game.board().coord_to_actor_id(&coord)));
            coord = dir.apply_to(coord);
            result
        })
    }

    fn actor_push(&mut self, actor_id: ActorId, rel_dir: RelDir) {
        let view = self.game.actor_view(&actor_id).unwrap();
        let push_dir = view.actor.looks_to.apply_relative(rel_dir);

        let coord = view.actor.coord;

        let pushed_actor_coord = view.actor.looks_to.apply_to(coord);

        let actor_line = self.actor_line(pushed_actor_coord, push_dir);
        let mut can_push = true;
        let mut pushed_actors = vec![];
        for (coord, actor_id) in actor_line {
            if let Some(actor_id) = actor_id {
                let pushed_view = self.game.actor_view(&actor_id).unwrap();
                if pushed_view.actor_type.pushable {
                    pushed_actors.push((actor_id, push_dir.apply_to(coord)));
                } else {
                    can_push = false;
                    break;
                }
            } else {
                break;
            }
        }
        // try push
        self.push_cmd(Cmd::TryPush(Dest {
            from_actor_id: actor_id,
            to_coord: pushed_actor_coord,
        }));
        for (actor_id, to_coord) in &pushed_actors {
            self.push_cmd(Cmd::TryPush(Dest {
                from_actor_id: *actor_id,
                to_coord: *to_coord,
            }));
        }
        if can_push {
            // complete push and active pushed actors
            for (actor_id, to_coord) in pushed_actors.iter().rev() {
                self.activate(*actor_id);
                self.game.update_actor(actor_id, |actor| {
                    actor.coord = *to_coord;
                });
                self.push_cmd(Cmd::CompletePush(Dest {
                    from_actor_id: *actor_id,
                    to_coord: *to_coord,
                }));
            }
            self.game.update_actor(&actor_id, |actor| {
                actor.coord = pushed_actor_coord;
            });
            self.push_cmd(Cmd::CompletePush(Dest {
                from_actor_id: actor_id,
                to_coord: pushed_actor_coord,
            }));
        } else {
            // cancel pushes
            for (actor_id, to_coord) in pushed_actors.iter().rev() {
                self.push_cmd(Cmd::CancelPush(Dest {
                    from_actor_id: *actor_id,
                    to_coord: *to_coord,
                }));
            }
            // still activate first though
            if let Some((actor_id, _to_coord)) = pushed_actors.first() {
                self.activate(*actor_id);
            }
            self.push_cmd(Cmd::CancelPush(Dest {
                from_actor_id: actor_id,
                to_coord: pushed_actor_coord,
            }));
        }
    }

    fn turn_actor(&mut self, actor_id: ActorId, rel_dir: RelDir) {
        self.game.update_actor(&actor_id, |actor| {
            actor.looks_to = actor.looks_to.apply_relative(rel_dir);
        });
        self.push_cmd(Cmd::Turn(actor_id, rel_dir));
    }

    fn process_actor_hits(&mut self, actor_id: ActorId, ivec2s: &[IVec2]) {
        let actor_view = self.game.actor_view(&actor_id).unwrap();
        let coord = actor_view.actor.coord;
        for hit_vec in ivec2s.iter().copied() {
            let actual_hit_vec = actor_view.actor.looks_to.rel_coord_to_coord(coord, hit_vec);
            self.push_cmd(Cmd::Hit(Dest {
                from_actor_id: actor_id,
                to_coord: actual_hit_vec,
            }));
            if let Some(hit_actor_id) = self.game.board().coord_to_actor_id(&actual_hit_vec) {
                self.activate(hit_actor_id);
            }
        }
    }
}
