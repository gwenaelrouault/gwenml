#[derive(Copy, Clone, Debug)]
pub enum FighterEvent {
    Standing,
    WalkingRight,
    WalkingLeft,
    EndWalkingRight,
    EndWalkingLeft,
    Crouch,
    EndCrouch,
    Attack1,
    Attack2,
    EndAttack,
    Blocking,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RunAction {
    Standing,
    Walking,
    Crouch,
    CrouchPunch,
    Punch,
    MiddleKick,
    HighKick,
    Blocking,
    CrouchBlocking,
}

#[derive(Copy, Clone, Debug)]
pub enum ActionTempo {
    Infinite,
    Continu,
    Immediate,
}
#[derive(Copy, Clone, Debug)]
pub enum Attack {
    NoAttack,
    Punch,
    HighKick,
    MiddleKick,
}