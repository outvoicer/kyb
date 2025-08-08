KYB server to determine Latvian members of boards.

Send person name (order: lastname + firstname), personal id and company registration code.
Receive confirmation, that this person is on the member of board of organisation.
Log server results.

# Source
https://dati.ur.gov.lv/officers/officers.csv

# Considerations:
+ Every role in this table is considered as a positive match (Also Liqvidators and others)
+ Name and reg code have to be exact matches (Bērziņš Jānis, not Berzins Janis)
+ If entry has personal code, then personal code is matched.
+ If entry does not have personal code in db (for foreign owners), then personal code is not used for validation.

# Build
```
cargo build --release
```

# Run
```
cargo run --release
Runs at 127.0.0.1:10001
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

# Test
```
cargo watch -x "test"
cargo watch -x "test query -- --nocapture"
cargo watch -x "test test_joonas_without_personal_code_in_db_is_valid -- --nocapture"
```

# Responses
```
200 - { valid: true, verfication_id: 5 }
417 Expectation failed - Either is not member of board or data was incomplete
501 "Server down"
```


# Curl
curl -X POST http://127.0.0.1:10001/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'

curl -X POST http://127.0.0.1:10001/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Sernjuk Joonas", "personal_code": "38303224942", "reg_code": "40203537286"}'

 curl -X POST http://127.0.0.1:10001/lv -H "Content-Type: application/json" -d '{"name": "Marots Jānis", "personal_code": "191282-*****", "reg_code": "40203278421"}' -vvv

 curl -X POST https://in.costpocket.com:8443/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Marots Jānis", "personal_code": "191282-*****", "reg_code": "40203278421"}' -vvv


 curl -X POST http://in.costpocket.com:8443/lv \
     -H "Content-Type: application/json" \
     -d '{"name": "Marots Jānis", "personal_code": "191282-*****", "reg_code": "40203278421"}' -vvv
