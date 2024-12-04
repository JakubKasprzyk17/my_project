use std::cell::RefCell;

thread_local! {
    static TO_DO_LIST: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    let a = String::from("CZESC");
    let b = String::from("ELO");
    let c = a;
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn greet_update(name: String) {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow_mut().push(name);
    });
}

#[ic_cdk::query]
fn get_greet_update() -> Vec<String> {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow().clone()
    })
}


#[ic_cdk::query]
fn add_to_do_item(item: String) {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow_mut().push(item);
    });
}

#[ic_cdk::query]
fn get_to_do_list() -> Vec<String> {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow().clone()
    })
}

#[ic_cdk::query]
fn clear_to_do_list() {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow_mut().clear();
    });
}

#[ic_cdk::query]
fn remove_to_do_item(index: usize) {
    TO_DO_LIST.with(|to_do_list| {
        to_do_list.borrow_mut().remove(index);
    });
}