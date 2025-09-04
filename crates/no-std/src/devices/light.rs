use picoserve::routing::{
    MethodHandler, NoPathParameters, NotFound, PathDescription, PathRouter, Router,
};

use crate::device::Device;

pub struct Light<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = NoPathParameters,
> {
    //routes: Vec<Route>,
    router: Router<PR, State, CurrentPathParameters>,
}

impl Default for Light<NotFound, (), NoPathParameters> {
    fn default() -> Self {
        Self::new()
    }
}

impl Light<NotFound, (), NoPathParameters> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            //routes: Vec::new(),
            router: Router::new(),
        }
    }
}

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    Light<PR, State, CurrentPathParameters>
{
    pub fn route<PD: PathDescription<CurrentPathParameters>>(
        self,
        path_description: PD,
        handler: impl MethodHandler<State, PD::Output>,
    ) -> Light<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters> {
        let new_router = self.router.route(path_description, handler);

        Light {
            //routes: self.routes,
            router: new_router,
        }
    }

    pub fn build(self) -> Device<PR, State, CurrentPathParameters> {
        Device::new(self.router)
    }
}
