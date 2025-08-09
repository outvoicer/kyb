Server to determine Latvian members of boards.

Send person name, personal id and company registration code.
Receive confirmation, when this person is a member of board.

Updates itself every night for Latvian Open Data portal.

Logs verification results and errors.

# Source
https://dati.ur.gov.lv/officers/officers.csv

# Considerations:
+ Person name order is lastname + firstname.
+ Person name has to be exact match (Bērziņš Jānis, not Berzins Janis)
+ Reg code has to be exact match and 11 characters (cannot have LV in beginning).
+ Every role in the CSV is considered as a positive match (Also Liqvidators and empty)
+ If CSV entry has personal code, then personal code is matched.
+ If CSV entry does not have personal code in db (like foreign owners), then personal code is not used for validation.

# Run
```
cargo run
Runs by default at 127.0.0.1:10001
```

# Install DB
```
cargo run install
or
./kyb install
```

# Try
```
curl -X POST http://127.0.0.1:10001/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'
```

# Response
```
200 - { valid: true, verfication_id: 5 }
417 - Expectation failed - Either is not member of board or data was incomplete
501 - "Server down"
Empty response - I have used .unwrap()

```

# Config
```
/src/config.rs
here you can set server address + port and schedule db update time.
```

# Build
```
// sudo apt-get install libsqlite3-dev
cargo build --release
```

# Test
```
cargo watch -x "test"
cargo watch -x "test query -- --nocapture"
cargo watch -x "test test_two_janises -- --nocapture"
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
Then:
```
sudo systemctl enable kyb
sudo systemctl start kyb
```

# Curl
curl -X POST http://127.0.0.1:10001/lv -H "Content-Type: application/json" -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'

 curl -X POST https://kyb.outvoicer.com/lv -H "Content-Type: application/json" -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'
