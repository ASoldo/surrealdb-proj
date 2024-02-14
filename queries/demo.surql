//
BEGIN;
LET $username = "Andrija";
SELECT * FROM users;

UPDATE users SET name = $username WHERE id = "users:l42i2oltmqw4ze798m42" RETURN AFTER;
COMMIT;

//
BEGIN;
CREATE human:andrijaa SET name = "Andrija", age = 30;

CREATE human:yukaa SET name = "Yuka", age = 20;
COMMIT;

//
UPDATE human SET skill = ["developer", "artist", "musician", "ceo"] WHERE id = "human:andrija";

SELECT * FROM human;

UPDATE human:andrija SET bff = human:yuka;
UPDATE human:yuka SET bff = human:andrija;

SELECT bff.name, bff.age FROM human:andrija;
SELECT bff.name, bff.age FROM human:yuka;

CREATE car:tesla SET model = "Model S", ev = true, price = 10000;
CREATE car:jeep SET model = "Wrangler", ev = false, price = 5000;

UPDATE human:andrija SET cars = ["car:tesla", "car:jeep"];

UPDATE car:tesla SET owner = human:andrija;
UPDATE car:jeep SET owner = human:yuka;

SELECT cars FROM human:andrija;
SELECT * FROM car WHERE owner == human:andrija;

CREATE part:tire SET brand = "Michelin", size = 5;
CREATE part:gastank SET brand = "Tanksy", size = 10;
CREATE part:battery SET brand = "Xi Ping", size = 15;

UPDATE car:jeep SET parts = ["part:tire", "part:gastank"];
UPDATE car:tesla SET parts = ["part:tire", "part:battery"];

SELECT parts FROM car:jeep;
SELECT cars.parts.brand FROM human:andrija;

// 
RELATE human:andrija->drove->car:jeep SET when = time::now(), destination = "the club";
RELATE human:yuka->drove->car:tesla SET when = time::now(), destination = "the library";

SELECT ->drove->car FROM human;
SELECT <-drove<-human.name,<-drove.destination, <-drove.when FROM car;

SELECT * FROM drove;
