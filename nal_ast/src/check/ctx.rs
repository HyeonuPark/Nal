use super::Error as E;

#[derive(Debug, Default)]
pub struct Ctx {
    error_list: Vec<E>,
    within_fn: bool,
    within_loop: bool,
}

impl Ctx {
    pub fn report(&mut self, err: E) {
        self.error_list.push(err);
    }

    pub fn errors(self) -> Vec<E> {
        self.error_list
    }

    pub fn with_fn<F: FnOnce(&mut Self)>(&mut self, sub: F) {
        let prev_fn = self.within_fn;
        let prev_loop = self.within_loop;

        self.within_fn = true;
        self.within_loop = false;

        sub(self);

        self.within_fn = prev_fn;
        self.within_loop = prev_loop;
    }

    pub fn with_loop<F: FnOnce(&mut Self)>(&mut self, sub: F) {
        let prev_loop = self.within_loop;
        self.within_loop = true;

        sub(self);

        self.within_loop = prev_loop;
    }

    pub fn is_fn(&self) -> bool {
        self.within_fn
    }

    pub fn is_loop(&self) -> bool {
        self.within_loop
    }
}
