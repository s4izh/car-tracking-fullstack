pragma solidity ^0.5.0;


contract CarKilometerCertification {
constructor() public {
    initCertifications();
}

function initCertifications() public {
    // Aquí es donde puedes agregar los certificados iniciales
    // utilizando la función `certifyKilometer` del contrato.
    //estos certificados se usan para no inicalizar la blockchain vacia y probar el frontend
    // Por ejemplo:
    certifyKilometer("ABC123", 20000);
    certifyKilometer("DEF456", 200000);
    certifyKilometer("QWER789", 200000);
}

    struct Car {
        string  licensePlate;
        uint256  kilometer;
        uint256  certifiedTimestamp;
    }

    mapping (string => Car) cars;

    //Certifica un nuevo kilometraje con su timestamp correspondiente
    function certifyKilometer(string memory licensePlate, uint256 currentKilometer) public {
        if (bytes(cars[licensePlate].licensePlate).length != 0) {
            cars[licensePlate] = Car(licensePlate, currentKilometer, block.timestamp);
        }
        else {
            require(currentKilometer >= cars[licensePlate].kilometer, "New kilometer value should not be less than previous value");
            cars[licensePlate].kilometer = currentKilometer;
            cars[licensePlate].certifiedTimestamp = block.timestamp;
        }
    }

    //devuelve los kilometros actuales del coche
    function getCarKilometer(string memory licensePlate) public view returns (uint256) {
        return cars[licensePlate].kilometer;
    }

    //Devuelve el timestamp de la última certificación
    function getCarCertificationTimestamp(string memory licensePlate) public view returns (uint256) {
        return cars[licensePlate].certifiedTimestamp;
    }

    function getCarCertification(string memory _licensePlate) public view returns (uint256, uint256) {
    return (cars[_licensePlate].kilometer, cars[_licensePlate].certifiedTimestamp);
    }

}


