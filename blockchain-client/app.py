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


def send_certificate_request(matricula, km):
    session = requests.Session()  # as defined in https://github.com/ethereum/wiki/wiki/JSON-RPC#net_version
    method = 'net_version'
    params = []
    payload= {"jsonrpc": "2.0",
              "method": method,
              "params": params,
              "id": 1}
    headers = {'Content-type': 'application/json'}

    # poner la dirección de entorno en el docker-compose
    blockchain_url = os.environ['BLOCKCHAIN_URL']

    # response = session.post(blockchain_url,  # dirección de la blockchain
    #                         json=payload, headers=headers)
    # print('raw json response: {}'.format(response.json()))
    # print('network id: {}'.format(response.json()['result']))


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
