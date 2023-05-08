from flask import Flask, request
import requests  # create persistent HTTP connection
import os
import web3
import time
import traceback

app = Flask(__name__)

# create persistent HTTP connection
session = requests.Session()

w3 = web3.Web3()


URL= "http://localhost:7545"



''' =========================== SOME FUNCTIONS ============================ '''
# see http://www.jsonrpc.org/specification
# and https://github.com/ethereum/wiki/wiki/JSON-RPC

def createJSONRPCRequestObject(_method, _params, _requestId):
    return {"jsonrpc":"2.0",
            "method":_method,
            "params":_params, # must be an array [value1, value2, ..., valueN]
            "id":_requestId}, _requestId+1
    
def postJSONRPCRequestObject(_HTTPEnpoint, _jsonRPCRequestObject):
    response = session.post(_HTTPEnpoint,
                            json=_jsonRPCRequestObject,
                            headers={'Content-type': 'application/json'})

    return response.json()



''' ================= SEND A TRANSACTION TO SMART CONTRACT  ================'''
def send_certificate_request(matricula, km):

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

    #cuenta disponible generada por ganache
    myAddress = '0xa15932e971a00e91E68bA168b617aE3a061aE8f2'
    myPrivateKey = '0x4f88a384b563428fab14df0c6593bbf0c3f263d32776c291d9f1aedafa3ffaa2'

    requestId = 1
    requestObject, requestId = createJSONRPCRequestObject('eth_getTransactionCount', [myAddress, 'latest'], requestId)
    responseObject = postJSONRPCRequestObject(URL, requestObject)
    nonce = w3.toInt(hexstr=responseObject['result'])
    
    # Construye los datos de la transacción
    function_signature = 'certifyKilometer(string,uint256)'
    methodId = w3.sha3(text=function_signature)[0:4].hex()
    function_encoded = contract.encode_function_call(fn_name=function_signature, args=function_inputs)
    param1 = (matricula).to_bytes(32, byteorder='big').hex()
    param2 = (km).to_bytes(32, byteorder='big').hex()
    data = '0x' + methodId + param1 + param2

    # Construye la transacción firmada
    transaction = {
        'nonce': nonce,
        'gasPrice': 1,
        'gas': 100000,
        'to': contract_address,
        'value': 0,
        'chainId': 1337,
        'data': function_encoded,
    }
    signed_txn = w3.eth.account.sign_transaction(transaction, private_key=myPrivateKey)
    params = [signed_txn.rawTransaction.hex()]

    # Envía la transacción a la red de blockchain
    requestObject, requestId = createJSONRPCRequestObject('eth_sendRawTransaction', params, requestId)
    responseObject = postJSONRPCRequestObject(URL, requestObject)
    transactionHash = responseObject['result']

    ### wait for the transaction to be mined
    while(True):
    requestObject, requestId = createJSONRPCRequestObject('eth_getTransactionReceipt', [transactionHash], requestId)
    responseObject = postJSONRPCRequestObject(URL, requestObject)
    receipt = responseObject['result']
    if(receipt is not None):
        if(receipt['status'] == '0x1'):
            print('transaction successfully mined')
            break
        else:
            pp.pprint(responseObject)
            raise ValueError("Error al esperar la confirmación de la transacción")
    time.sleep(1)



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