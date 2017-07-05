pub struct Container<L: Liquid<L>> {
    size: i32,
    name: &'static str,
    liquids: Vec<L>,
}

impl<L> Container<L>
    where
    L: Liquid<L>,
{
    fn contents(&self) -> i32 {
        self.liquids
            .iter()
            .fold(0, |sum, x| sum + x.quantity())
    }
    fn is_full(&self) -> bool { self.contents() < self.size }

    fn can_fit(&self, liq: &L) -> bool {
        self.contents() + liq.quantity() < self.size
    }

    fn is_empty(&self) -> bool { self.liquids.len() == 0 }

    fn fill(&mut self, liq: L) -> bool {
        if self.can_fit(&liq) {
            self.liquids.push(liq);
            true
        } else if self.is_full() {
            false
        } else {
            let nliq = liq.new(self.size - self.contents());
            self.fill(nliq)
        }
    }
}

pub trait Liquid<L: Liquid<L>> {
    fn quantity(&self) -> i32;
    fn amount(&self) -> i32;
    fn new(&self, quantity: i32) -> L;
}
