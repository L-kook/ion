# cargo watch \
#     -w crates \
#     -w examples \
#     -w _scratch \
#     -- bash -c "\
#         clear && \
#         cargo build --package ion_scratch && \
#         ./target/release/ion_scratch.exe \
#     "

cargo watch \
    -w crates \
    -w examples \
    -w _scratch \
    -- bash -c "\
        clear && \
        cargo build --package ion_examples && \
        ./target/debug/ion_examples basic && \
        echo -------- \
    "