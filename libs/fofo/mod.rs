pub trait StateLeaf {
    type Escalation;
    type Action;
    type Mutation;
    fn resolve(&self,action: Self::Action) -> Update<Self::Escalation,Self::Mutation>;
    fn update(&mut self,mutation: Self::Mutation);
}

pub struct StoreLeaf<State: StateLeaf> {
    state: State
}

impl <State: StateLeaf> StoreLeaf<State> {
    pub fn dispatch(&mut self,action:State::Action) {
        let update = self.state.resolve(action);
        match update {
            Update::Escalate(_) => todo!(),
            Update::This(mutation) => {
                self.state.update(mutation)
            },
        }
    }
}

pub enum Update<E,M> {
    Escalate(E),
    This(M)
}

enum SampleRoot {
    Leaf1(SampleLeaf1),
    Leaf2(SampleLeaf2)
}

struct SampleLeaf1 {

}



struct SampleLeaf2 {

}