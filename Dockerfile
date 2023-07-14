FROM rust:1.67

WORKDIR /projects/matrix_calculator
COPY . .

RUN cargo install --path .

CMD ["matrix_calculator"]