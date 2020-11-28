pub fn clrscr() {
    print!("\x1B[2J\x1B[1;1H"); // clears screen
}
