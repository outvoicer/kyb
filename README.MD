KYB server to determine Latvian members of boards.

Send person name (order: lastname + firstname), personal id and company registration code.
Receive confirmation, that this person is on the member of board of organisation.

Updates itself every night for Latvian Open Data portal.

Logs server results.

# Source
https://dati.ur.gov.lv/officers/officers.csv

# Considerations:
+ Every role in the CSV is considered as a positive match (Also Liqvidators and others)
+ Name has to be exact match (Bērziņš Jānis, not Berzins Janis)
+ Reg code has to be exact match
+ If CSV entry has personal code, then personal code is matched.
+ If entry does not have personal code in db (like foreign owners), then personal code is not used for validation.

# Run
```
cargo run
Runs at 127.0.0.1:10001
```

# Install DB
```
./kyb install
or
cargo run install
```

# Build
```
cargo build --release
```

# Test
```
cargo watch -x "test"
cargo watch -x "test query -- --nocapture"
cargo watch -x "test test_two_janises -- --nocapture"
```

# Responses
```
200 - { valid: true, verfication_id: 5 }
417 Expectation failed - Either is not member of board or data was incomplete
501 "Server down"
```

# DB
```
sqlite3 /home/www/.local/share/kyb/kyb.db
sqlite3 /Users/martin/Library/Application\ Support/kyb/kyb.db
```

# Deploy
```
sudo nano /etc/systemd/system/kyb.service

[Unit]
Description=KYB Server
After=network.target

[Service]
User=www
ExecStart=/home/www/kyb/target/release/kyb
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

# Curl
curl -X POST http://127.0.0.1:10001/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'

curl -X POST http://127.0.0.1:10001/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Sernjuk Joonas", "personal_code": "38303224942", "reg_code": "40203537286"}'

 curl -X POST http://127.0.0.1:10001/lv -H "Content-Type: application/json" -d '{"name": "Marots Jānis", "personal_code": "191282-*****", "reg_code": "40203278421"}' -vvv

 curl -X POST https://kyb.costpocket.com/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Marots Jānis", "personal_code": "191282-*****", "reg_code": "40203278421"}' -vvv
