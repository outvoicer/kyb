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
All fields must be strings.

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

# Misc
```
All companies in Latvia:
curl https://dati.ur.gov.lv/register/register.csv

Public institutions:
curl https://dati.ur.gov.lv/register/ppi_public_persons_institutions.csv

VAT payers:
curl https://data.gov.lv/dati/dataset/9a5eae1c-2438-48cf-854b-6a2c170f918f/resource/610910e9-e086-4c5b-a7ea-0a896a697672/download/pdb_pvnmaksataji_odata.csv

```

# Company
```
Company::search_by_name(&Connection, &String)
Company::create_table(conn)

curl -X POST http://127.0.0.1:10001/lv/company -H "Content-Type: application/json" -d '{"name": "Raimond"}' -vvv

```


# Considerations - Company registry
There are companies with a registration code 000703116, which seams unsafe to store as number and we use string.
search limit 10
Search "", get []
http: Errors are not returned, [] is returned
longest name is 151 chars: "JURIDISKU SŪDZĪBU RAKSTĪŠANAS BIEDRĪBA, BIZNESA UN JURIDISKĀS KONSULTĀCIJAS, PĀRSTĀVĪBA, AIZSTĀVĪBA, CILVĒKTIESĪBU AIZSTĀVĪBA, BĒRNU UN SPORTA PASĀKUMI"

Original size: 125 MB
Only name, "SIA", address and index: 45 MB
+ only existing companies: 26 mb
