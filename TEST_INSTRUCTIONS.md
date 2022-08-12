# Testing

At root of this repo:
```
yarn
anchor build --skip-lint
./copy_so.sh
anchor test --skip-lint
```

This will run bubblegum-test.ts but it will not pass.  This code only provided as debugging reference.


# API generation

This is working fine.  Navigate to bubblegum/js and run `yarn api:gen`.