# exec cargo watch \
#     -w crates \
#     -w examples \
#     -w _scratch \
#     -- bash -c "\
#         clear && \
#         cargo build --package ion_scratch && \
#         ./target/debug/ion_scratch && \
#         echo -------- \
#     "

exec cargo watch \
    -w crates \
    -w examples \
    -w _scratch \
    -- bash -c "\
        clear && \
        cargo build --package ion_cli && \
        ./target/debug/ion_cli run ./examples/js/modules/index.js && \
        echo -------- \
    "

# cargo watch \
#     -w crates \
#     -w examples \
#     -w _scratch \
#     -- bash -c "\
#         clear && \
#         cargo build --package ion_examples && \
#         ./target/debug/ion_examples basic && \
#         echo -------- \
#     "