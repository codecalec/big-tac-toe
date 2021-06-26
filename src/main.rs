use big_tac_toe::{ Marking, OuterBoard };

fn main() {

    let nought = Marking::Nought;
    let cross = Marking::Cross;

    let mut outer = OuterBoard::new();
    outer.place(0, 0, &0, &1, &cross);
    outer.place(0, 0, &0, &0, &cross);
    outer.place(0, 0, &0, &2, &cross);

    outer.place(0, 1, &0, &1, &cross);
    outer.place(0, 1, &0, &0, &cross);
    outer.place(0, 1, &0, &2, &cross);

    outer.place(0, 2, &0, &1, &cross);
    outer.place(0, 2, &0, &0, &cross);
    outer.place(0, 2, &0, &0, &cross);
    //outer.place(0, 2, &0, &2, &cross);

    outer.place(2, 2, &2, &2, &nought);

    println!("{}", outer);
    println!("{}", outer.master_board);
}
