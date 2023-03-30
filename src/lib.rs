use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
#[derive(Copy)]
enum Reset {
    NewlinesReset,
    SpacesReset,
    NoReset,
}


#[pyclass]
struct Counter {
    what: char,
    min_number: u64,
    reset: Reset,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(what: char, min_number: u64, reset: Reset) -> Self {
        Counter{what: what, min_number: min_number, reset: reset}
    }

    fn has_count(
        &self,
        data: &str,
    ) -> bool {
        has_count(self, data.chars())
    }
}


fn has_count(cntr: &Counter, chars: std::str::Chars) -> bool {
    let mut current_count : u64 = 0;
    for c in chars {
        if got_count(cntr, c, &mut current_count) {
            return true;
        }
    }
    false
}

fn got_count(cntr: &Counter, c: char, current_count: &mut u64) -> bool {
    maybe_reset(cntr, c, current_count);
    maybe_incr(cntr, c, current_count);
    *current_count >= cntr.min_number
}

fn maybe_reset(cntr: &Counter, c: char, current_count: &mut u64) -> () {
    match (c, cntr.reset) {
        ('\n', Reset::NewlinesReset) | (' ', Reset::SpacesReset)=> {
            *current_count = 0;
        }
        _ => {}
    };
}

fn maybe_incr(cntr: &Counter, c: char, current_count: &mut u64) -> (){
    if c == cntr.what {
        *current_count += 1;
    };
}

#[pymodule]
fn counter(_py: Python, m: &PyModule
) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<Reset>()?;
    Ok(())
}