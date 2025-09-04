use picoserve::routing::{NoPathParameters, PathRouter, Router};

pub struct Device<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = NoPathParameters,
> {
    //routes: Vec<Route>,
    pub(crate) router: Router<PR, State, CurrentPathParameters>,
}

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    Device<PR, State, CurrentPathParameters>
{
    pub(crate) fn new(router: Router<PR, State, CurrentPathParameters>) -> Self {
        Self { router }
    }
}
