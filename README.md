# rudoku

```
cargo run cli --solve data/sample1.txt sample1_solved.txt
cargo run tui --create
```

Rust Sudoku library and command line interface (CLI). The library consists of abstractions for the sudoku board, along with sudoku solvers and generators that use a variety of different algorithsm. The CLI consists of commands to solve and generate sudoku boards, and also a TUI based on ratatui for interactive gameplay. 

### TODO:
- [X] Sudoku solver (backtracking)
- [ ] Sudoku solver (genetic algo)
- [ ] Sudoku solver (exact cover)
- [ ] Sudoku solver (constraint)
- [X] Sudoku board generator
- [ ] Interactive sudoku TUI
- [ ] Actually write tests
