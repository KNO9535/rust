use mini_tasks::board::Board;
use mini_tasks::task::Task;
use mini_tasks::task::Priority;

fn main() {
    let mut board = Board::new();
    board.add_task("Nauka", Task::new("Powtorzyc hashmap", Priority::High));
    board.add_task("Nauka", Task::new("Powtorzyc listy", Priority::High));
    board.add_task("Dom", Task::new("Wyniesc smieci", Priority::Low));
    board.add_task("Dom", Task::new("Umyc podlogi", Priority::Medium));
   
    println!(
        "Ukończono istniejące: {}",
        board.complete_task("Nauka", "Powtorzyc hashmap")
    );
    println!(
        "Ukończono nieistniejące: {}",
        board.complete_task("Dom", "Umyc podlogi")
    );

    board.print_list("Nauka");
    board.print_list("Dom");
    board.print_list("Praca");

    println!("Nieukończone zadania: {}", board.count_pending());
}

