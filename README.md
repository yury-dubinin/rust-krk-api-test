# rust-krk-api-test

This repo is to show simple usage of Rust and Cucumber for an API testing.
Test features are created in Gherkin(Given/When/Then) style. Test steps are implimented in the most simple way but run asynchronosly.
Following test cases are included:
- Requests and reports the information for the Time, validate response (no errors)
- Requests and reports the information for the XBT/USD trading pair, validate response (no errors)
- Connects to an account for private API and Requests and reports all open orders on the account.

### Extra features

- GitHub actions workflow to run the tests
- Dockerfile

## How to use 
To run tests locally you would need to have the Rust installed and have a Krk API credentials.

Environment variables: 
- API_KEY
- PRIVATE_KEY

Local execution:
- Build frozen dependencies with `cargo build --locked`
- Run the tests with `cargo test --test api`

Remote execution (recomended):
- Go to GitHub [workflow](https://github.com/yury-dubinin/rust-krk-api-test/actions/workflows/rust.yml)
- Click `Run workflow` on main branch
- Navigate to [action run](https://github.com/yury-dubinin/rust-krk-api-test/actions/runs/9700028916)

To build a docker image:
- With Docker running execute -> `docker build -t rust_project .`
