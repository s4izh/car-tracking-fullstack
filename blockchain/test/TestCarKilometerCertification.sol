pragma solidity ^0.5.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/CarKilometerCertification.sol";

contract TestCarKilometerCertification {

  function testRegisterCar() public {
    CarKilometerCertification carKilometerCertification = CarKilometerCertification(DeployedAddresses.CarKilometerCertification());

    // Registro de un nuevo coche
    carKilometerCertification.registerCar("ABC123", 10000);

    uint expectedKilometer = 10000;
    uint expectedTimestamp = block.timestamp;

    Assert.equal(carKilometerCertification.getCarKilometer("ABC123"), expectedKilometer, "El kilometraje registrado debe ser 10000");
    Assert.equal(carKilometerCertification.getCarCertificationTimestamp("ABC123"), expectedTimestamp, "La fecha de certificación debe ser la actual");
  }

  function testCertifyKilometer() public {
    CarKilometerCertification carKilometerCertification = CarKilometerCertification(DeployedAddresses.CarKilometerCertification());

    // Registro de un nuevo coche
    carKilometerCertification.registerCar("DEF456", 20000);

    // Certificación del kilometraje
    carKilometerCertification.certifyKilometer("DEF456", 25000);

    uint expectedKilometer = 25000;
    uint expectedTimestamp = block.timestamp;

    Assert.equal(carKilometerCertification.getCarKilometer("DEF456"), expectedKilometer, "El kilometraje registrado debe ser 25000");
    Assert.equal(carKilometerCertification.getCarCertificationTimestamp("DEF456"), expectedTimestamp, "La fecha de certificación debe ser la actual");
  }

  /*function testChangeCarOwner() public {
    CarKilometerCertification carKilometerCertification = CarKilometerCertification(DeployedAddresses.CarKilometerCertification());

    // Registro de un nuevo coche
    carKilometerCertification.registerCar("GHI789", 30000);

    // Cambio de propietario del coche
    address newOwner = address(0x123);
    carKilometerCertification.changeCarOwner("GHI789", newOwner);

    address expectedOwner = newOwner;

    Assert.equal(carKilometerCertification.cars("GHI789").owner, expectedOwner, "El nuevo propietario del coche debe ser la dirección 0x123");
  }*/

}
