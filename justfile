# "just create 1" to start day01 part1

create day:
    ./scripts/set_part1.sh {{day}}
    ./scripts/get_input.sh {{day}}

done_part1 day:
    ./scripts/done_part1.sh {{day}}

part2 day:
    ./scripts/set_part2.sh {{day}}

done_part2 day:
    ./scripts/done_part2.sh {{day}}

test:
    cargo run
    cargo test

done day:
    ./scripts/done_part1.sh {{day}}
    ./scripts/done_part2.sh {{day}}
