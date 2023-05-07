from flask import Flask, request
import requests  # create persistent HTTP connection
import os
import web3
import time

app = Flask(__name__)

# create persistent HTTP connection
session = requests.Session()
w3 = web3.Web3()


requestId = 0  # is automatically incremented at each request

myAccount = w3.eth.account.create('entropia que te cagas')
myAddress = myAccount.address
myPrivateKey = myAccount._private_key
print('my address is     : {}'.format(myAccount.address))
print('my private key is : {}'.format(myPrivateKey.hex()))
URL= "http://localhost:7545"

account_1 = w3.eth.account.create('entropia que te cagas')
myAddress = account_1.address
private_key1 = account_1._private_key
contract_address = '0x24AfeC5C0CaFD7410d3dFAfeBc1DeCDF3c6b5BB2'
contract_abi = ([
        {
          "inputs": [],
          "payable": false,
          "stateMutability": "nonpayable",
          "type": "constructor"
        },
        {
          "constant": false,
          "inputs": [],
          "name": "initCertifications",
          "outputs": [],
          "payable": false,
          "stateMutability": "nonpayable",
          "type": "function"
        },
        {
          "constant": false,
          "inputs": [
            {
              "internalType": "string",
              "name": "licensePlate",
              "type": "string"
            },
            {
              "internalType": "uint256",
              "name": "currentKilometer",
              "type": "uint256"
            }
          ],
          "name": "certifyKilometer",
          "outputs": [],
          "payable": false,
          "stateMutability": "nonpayable",
          "type": "function"
        },
        {
          "constant": true,
          "inputs": [
            {
              "internalType": "string",
              "name": "licensePlate",
              "type": "string"
            }
          ],
          "name": "getCarKilometer",
          "outputs": [
            {
              "internalType": "uint256",
              "name": "",
              "type": "uint256"
            }
          ],
          "payable": false,
          "stateMutability": "view",
          "type": "function"
        },
        {
          "constant": true,
          "inputs": [
            {
              "internalType": "string",
              "name": "licensePlate",
              "type": "string"
            }
          ],
          "name": "getCarCertificationTimestamp",
          "outputs": [
            {
              "internalType": "uint256",
              "name": "",
              "type": "uint256"
            }
          ],
          "payable": false,
          "stateMutability": "view",
          "type": "function"
        },
        {
          "constant": true,
          "inputs": [
            {
              "internalType": "string",
              "name": "_licensePlate",
              "type": "string"
            }
          ],
          "name": "getCarCertification",
          "outputs": [
            {
              "internalType": "uint256",
              "name": "",
              "type": "uint256"
            },
            {
              "internalType": "uint256",
              "name": "",
              "type": "uint256"
            }
          ],
          "payable": false,
          "stateMutability": "view",
          "type": "function"
        }
      ])

def send_certificate_request(matricula, km):
    ganache_url = 'http://localhost:7545'
    web3 = Web3(Web3.HTTPProvider(ganache_url))

    # Create a Contract object
    contract = web3.eth.contract(address=contract_address, abi=contract_abi)
    
    #get the nonce.  Prevents one from sending the transaction twice
    nonce = web3.eth.getTransactionCount(account_1)

    # Encode the function call
    function = contract.functions.certifykilometer(matricula , km)
    data = function.encodeABI()

    #build a transaction in a dictionary
    tx = {
        'nonce': nonce,
        'to': contract_address,
        'data': data,
        'value': web3.toWei(1, 'ether'),
        'gas': 2000000,
        'gasPrice': web3.toWei('50', 'gwei')
    }

    #sign the transaction
    signed_tx = web3.eth.account.sign_transaction(tx, private_key1)

    #send transaction
    tx_hash = web3.eth.sendRawTransaction(signed_tx.rawTransaction)

    #get transaction hash
    print(web3.toHex(tx_hash))


@app.route('/')
def index():
    return 'Hello, World!', 200


# esto coge dos parametros así /certificate?matricula=1234&km=1234
@app.route('/certificate')
def certificate_handler():
    matricula = request.args.get('matricula', type=str)
    km = request.args.get('km', type=int)
    #send_certificate_request(matricula, km)
    return 'certification completed', 200


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
