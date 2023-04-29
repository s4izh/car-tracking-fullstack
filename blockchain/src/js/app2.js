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
        var CarKilometerCertificationArtifact = data;
        App.contracts.CarKilometerCertification = TruffleContract(CarKilometerCertificationArtifact);
  
        // Set the provider for our contract
        App.contracts.CarKilometerCertification.setProvider(App.web3Provider);
  
        return App.bindEvents();
      });
  
  
      return App.bindEvents();
    },
  
  
    //HANDLER CERTIFICACION
   bindEvents: function() {
      $(document).on('click', '#certificationBtn', App.handleCertification);
    },
  
    handleCertification: function(event) {
      event.preventDefault();
  
      var mileage = parseInt($(event.target).data('actualMileage'));
      var licensePlateInput = document.getElementById("licensePlateInput");

      var certificationInstance;
  
      web3.eth.getAccounts(function(error, accounts) {
        if (error) {
          console.log(error);
        }
  
        var account = accounts[0];
  
        App.contracts.CarKilometerCertification.deployed().then(function(instance) {
          certificationInstance = instance;
  
          // Execute certification as a transaction by sending account
          certificationInstance.certifyKilometer(mileage, licensePlateInput, {from: account});
        }).catch(function(err) {
          console.log(err.message);
        });
      });
  
    }
  
  };
  
  $(function() {
    $(window).load(function() {
      App.initWeb3();
    });
  });
  
  
  