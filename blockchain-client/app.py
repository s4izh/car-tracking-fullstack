from flask import Flask, request
import requests  # create persistent HTTP connection
import os
from web3 import Web3
import time
import traceback

app = Flask(__name__)

# create persistent HTTP connection
session = requests.Session()

''' ================= SEND A TRANSACTION TO SMART CONTRACT  ================'''
def send_certificate_request(matricula, km):
  URL = "http://blockchain:7545"
  contractAddress = '0x24AfeC5C0CaFD7410d3dFAfeBc1DeCDF3c6b5BB2'
  contractAbi = [ 
      {
      "inputs": [],
      "payable": False,
      "stateMutability": "nonpayable",
      "type": "constructor"
  },
  {
      "constant": False,
      "inputs": [],
      "name": "initCertifications",
      "outputs": [],
      "payable": False,
      "stateMutability": "nonpayable",
      "type": "function"
  },
      {
        "constant": False,
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
        "payable": False,
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "constant": True,
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
        "payable": False,
        "stateMutability": "view",
        "type": "function"
      },
      {
        "constant": True,
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
        "payable": False,
        "stateMutability": "view",
        "type": "function"
      },
      {
        "constant": True,
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
        "payable": False,
        "stateMutability": "view",
        "type": "function"
      }
  ]
  w3 = Web3(Web3.HTTPProvider(URL))

      #direccion y abi del contrato
  contractAddress = '0x24AfeC5C0CaFD7410d3dFAfeBc1DeCDF3c6b5BB2'
      #cuenta disponible generada por ganache
  myAddress = '0xa15932e971a00e91E68bA168b617aE3a061aE8f2'
  myPrivateKey = '0x4f88a384b563428fab14df0c6593bbf0c3f263d32776c291d9f1aedafa3ffaa2'

      # Obtener el nonce de la cuenta
  nonce = w3.eth.get_transaction_count(myAddress)

      # Obtiene el contrato y crea la transacción
  contract = w3.eth.contract(address=contractAddress, abi=contractAbi)
  encoded_function_call = contract.functions.certifyKilometer(matricula, km).build_transaction({
      'from': myAddress,
      'nonce': nonce,
      'gas': 2000000,
      'gasPrice': w3.to_wei('20', 'gwei'),
      'chainId': 1337
  })

      # Signa y envía la transacción
  signed_txn = w3.eth.account.sign_transaction(encoded_function_call, private_key = myPrivateKey)    
  tx_hash = w3.eth.send_raw_transaction(signed_txn.rawTransaction)
  tx_receipt = w3.eth.wait_for_transaction_receipt(tx_hash)

      # Verifica que la transacción fue procesada correctamente
  if tx_receipt.status == 1:
      print('Transacción exitosa')
  else:
      print('Transacción fallida')

@app.route('/')
def index():
    return 'Hello, World!', 200


# esto coge dos parametros así /certificate?matricula=1234&km=1234
@app.route('/certificate')
def certificate_handler():
    matricula = request.args.get('matricula', type=str)
    km = request.args.get('km', type=int)
    send_certificate_request(matricula, km)
    return 'certification completed', 200


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)

