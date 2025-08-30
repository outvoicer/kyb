Business tools for Latvian KYB procedures.

# Validate Latvian member of board
- Send person name, personal id and company registration code.
- Receive confirmation, when this person is a member of board.

# Search Latvian company or government institution
- REST API and websocket support.
- Search with company name.
- searches with or without Latvian special characters.
- case independent.
- lists precice match first
- finds name, reg code, vat number, city, address, zip
- results limited to 10 results

# Update and logging
- Keeps a local ~ 60 mb db of companies, public institutions and members of board
- Updates itself every night from Latvian Open Data portal.
- Logs verification results and errors.
- Logs searches and results

# Install DB
```
cargo run install
or
./kyb install
```

# Run
```
cargo run
Runs by default at 127.0.0.1:10001
```

# REST API company search:
```
curl -X POST http://127.0.0.1:10001/lv/company -H "Content-Type: application/json" -d '{"name": "Raimond fantastic "}'

Response:
[{"legal_form":"SIA","name":"Raimond fantastic","city":"Jūrmala","address":"Mellužu prospekts 76","zip":"2008","public_sector":"0","reg_code":"40203572370","vat":true,"vat_number":"LV40203572370"}]

Search "", get []
Errors are not returned, enpty array [] is returned.
```
# Websocket company search:
```
// assumes websocket client is installed as "ws"
ws ws://127.0.0.1:10001/lv/air
{"name": "Raimond fantastic"}

Response:
{"result":[{"legal_form":"SIA","name":"Raimond fantastic","city":"Jūrmala","address":"Mellužu prospekts 76","zip":"2008","public_sector":"0","reg_code":"40203572370","vat":true,"vat_number":"LV40203572370"}],"error":null}

or
{"name": "asdfadsf"}
{"result":[],"error":null}

{name: "Raimond fantastic"}     // missing "" around "name"
{"result":null,"error":"Failed to deserialize message"}

or
asdf
{"result":null,"error":"Failed to deserialize message"}

```

# Considerations - Company search and VAT
- Assumes reg code (40203572370) and VAT Number (LV40203572370) number part match.
- There are companies with a registration code 000703116, so we use string for this.

# Try member board validation
```
All fields must be strings.

curl -X POST http://127.0.0.1:10001/lv/board \
     -H "Content-Type: application/json" \
     -d '{"name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360"}'
```

# Response for member of board
```
200 - { valid: true, verfication_id: 5 }
417 - Expectation failed - {"error":"Not member of board. Error id: 1238"} - Either is not member of board or data was incomplete.
501 - "Server down"
Empty response - I have used .unwrap()
```

# Considerations for Members of board validation:
+ Only first part of personal_code is matched, therefore we have "201292-*****" in sample. Second part comes anonymised from source.
+ Person name order is lastname + firstname.
+ Person name has to be exact match (Bērziņš Jānis, not Berzins Janis)
+ Reg code has to be exact match and 11 characters (cannot have LV in beginning).
+ Every role in the CSV is considered as a positive match (Also Liqvidators and empty)
+ If CSV entry has personal code, then personal code is matched.
+ If CSV entry does not have personal code in db (like foreign owners), then personal code is not used for validation.

# Sources
```
// Members of board:
https://dati.ur.gov.lv/officers/officers.csv

// All companies in Latvia:
https://dati.ur.gov.lv/register/register.csv

// All public institutions in Latvia
https://dati.ur.gov.lv/register/ppi_public_persons_institutions.csv

All VAT payers in Latvia:
https://data.gov.lv/dati/dataset/9a5eae1c-2438-48cf-854b-6a2c170f918f/resource/610910e9-e086-4c5b-a7ea-0a896a697672/download/pdb_pvnmaksataji_odata.csv
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
Some tests require db to be installed and some require server to be running.
```
cargo test
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
User=myusername
ExecStart=/home/myusername/kyb/target/release/kyb
Restart=on-failure

[Install]
WantedBy=multi-user.target
```
Then:
```
sudo systemctl enable kyb
sudo systemctl start kyb
```
Logs:
```
journalctl -u kyb.service
```

# Company
```
Company::search_by_name(&Connection, &String)
Company::create_table(conn)

```
# Random
longest name is 151 chars: "JURIDISKU SŪDZĪBU RAKSTĪŠANAS BIEDRĪBA, BIZNESA UN JURIDISKĀS KONSULTĀCIJAS, PĀRSTĀVĪBA, AIZSTĀVĪBA, CILVĒKTIESĪBU AIZSTĀVĪBA, BĒRNU UN SPORTA PASĀKUMI"

Original size of company registry: 125 MB
Only name, "SIA", city, address, index: 45 MB
+ only existing companies: 26 mb
+ companies, gov instututions, VAT numbers and normalized name and all Latvian board member data: 56.4 MB
