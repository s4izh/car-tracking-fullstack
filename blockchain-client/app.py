from flask import Flask, request
import requests  # create persistent HTTP connection
import os
import web3

app = Flask(__name__)

w3 = web3.Web3()
myAccount = w3.eth.account.create('entropia que te cagas')
myAddress = myAccount.address
myPrivateKey = myAccount._private_key
print('my address is     : {}'.format(myAccount.address))
print('my private key is : {}'.format(myPrivateKey.hex()))
BLOCKCHAIN_URL= "http://localhost:7545"


def send_certificate_request(matricula, km):
    # Convierte la matrícula y los kilómetros en formato compatible con Solidity
    license_plate_bytes = matricula.encode('utf-8')
    kilometer = km

    # Conecta a la red de blockchain
    w3 = web3.Web3(web3.HTTPProvider(os.environ[BLOCKCHAIN_URL]))

    # Carga el contrato desde su dirección
    contract_address = web3.Web3.toChecksumAddress('0x24AfeC5C0CaFD7410d3dFAfeBc1DeCDF3c6b5BB2')
    contract_abi = [
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
      ]  # Abi del contrato
    contract = w3.eth.contract(address=contract_address, abi=contract_abi)

    # Construye los datos de la transacción
    function_signature = 'certifyKilometer(string,uint256)'
    function_inputs = [license_plate_bytes, kilometer]
    function_encoded = contract.encode_function_call(fn_name=function_signature, args=function_inputs)
    nonce = w3.eth.getTransactionCount(myAddress)
    gas_price = w3.eth.gasPrice
    gas_limit = 100000  # Este valor depende de la complejidad de la función
    value = 0  # No se requiere valor para esta transacción
    chain_id = w3.eth.chainId

    # Construye la transacción firmada
    transaction = {
        'nonce': nonce,
        'gasPrice': gas_price,
        'gas': gas_limit,
        'to': contract_address,
        'value': value,
        'chainId': chain_id,
        'data': function_encoded,
    }
    signed_txn = w3.eth.account.sign_transaction(transaction, private_key=myPrivateKey)

    # Envía la transacción a la red de blockchain
    tx_hash = w3.eth.sendRawTransaction(signed_txn.rawTransaction)
    tx_receipt = w3.eth.waitForTransactionReceipt(tx_hash)

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
