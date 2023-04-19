const CarKilometerCertification = artifacts.require("CarKilometerCertification");

module.exports = function(deployer) {
  deployer.deploy(CarKilometerCertification).then(function(instance){
    console.log("CarKilometerCertification deployed at address: ", instance.address);
  });
};

