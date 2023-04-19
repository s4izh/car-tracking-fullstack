var App = {
  web3Provider: null,
  contracts: {},

  initWeb3: async function() {
    //conectar a Ganache  
    App.web3Provider = new Web3.providers.HttpProvider('http://localhost:7545');
  
    web3 = new Web3(App.web3Provider);

    return App.initContract();
  },

  initContract: function() {
    $.getJSON('CarKilometerCertification.json', function(data) {
      // Get the necessary contract artifact file and instantiate it with @truffle/contract
      var CarKilometerCertificatinoArtifact = data;
      App.contracts.CarKilometerCertification = TruffleContract(CarKilometerCertificatinoArtifact);

      // Set the provider for our contract
      App.contracts.CarKilometerCertification.setProvider(App.web3Provider);

      // Use our contract to retrieve and mark the adopted pets
      return App.bindEvents();
    });


    return App.bindEvents();
  },


  //HACER CONSULTA
  bindEvents: function() {
    $(document).on('click', '#submitBtn', App.handleRequest);
  },

  handleRequest: function(event) {
    event.preventDefault();

    const licensePlate = licensePlateInput.value;
    var instance;

    App.contracts.CarKilometerCertification.deployed().then(function(inst) {
      instance = inst;
      
      return instance.getCarCertification(licensePlate);
    }).then(function(result) {
      const mileage = result[0];
      const date = new Date(result[1] * 1000).toLocaleDateString();
      // Actualiza la UI con los kilómetros obtenidos y la fecha
      //Si timestamp es 0 significa que no existe por lo que no se muestra
      if (result[1] != 0) {
        mileageValue.textContent = mileage;
        dateValue.textContent = date;
        licensePlateNotFound.textContent = "";
      }
      else {
        mileageValue.textContent = null;
        dateValue.textContent = null;
        licensePlateNotFound.textContent = "LicensePlate not found";
      }

      
    }).catch(function(err) {
      console.log(err.message);
    });
  },

  //HANDLER CERTIFICACION
 /*bindEvents: function() {
    $(document).on('click', '#certificationBtn', App.handleCertification);
  },

  handleCertification: function(event) {
    event.preventDefault();

    var mileage = parseInt($(event.target).data('actualMileage'));

    var certificationInstance;

    web3.eth.getAccounts(function(error, accounts) {
      if (error) {
        console.log(error);
      }

      var account = accounts[0];

      App.contracts.CarKilometerCertification.deployed().then(function(instance) {
        certificationInstance = instance;

        // Execute certification as a transaction by sending account
        certificationInstance.certifyKilometer(mileage, {from: account});
      }).catch(function(err) {
        console.log(err.message);
      });
    });

  }*/

};

$(function() {
  $(window).load(function() {
    App.initWeb3();
  });
});


