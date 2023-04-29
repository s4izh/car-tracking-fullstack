from flask import Flask, request
import requests  # create persistent HTTP connection
import os
import web3

app = Flask(__name__)

# w3 = web3.Web3()
# myAccount = w3.eth.account.create('entropia que te cagas')
# myAddress = myAccount.address
# myPrivateKey = myAccount._private_key

# print('my address is     : {}'.format(myAddress))
# print('my private key is : {}'.format(myPrivateKey))

# web3 = web3.Web3(web3.HTTPProvider(blockchain_url))

# blockchain_url = os.environ['BLOCKCHAIN_URL']
# my_contract_abi = os.environ['ABI']
# my_contract_address = os.environ['ADDRESS']
# my_contract = web3.eth.contract(address=my_contract_address, abi=my_contract_abi)


# def send_certificate_request(matricula, km):
#     certificate = my_contract.functions.certifyKilometer(matricula, km).call()


@app.route('/')
def index():
    return 'Hello, World!', 200


# esto coge dos parametros así /certificate?matricula=1234&km=1234
@app.route('/certificate')
def certificate_handler():
    matricula = request.args.get('matricula', type=str)
    km = request.args.get('km', type=int)
    # send_certificate_request(matricula, km)
    return 'certification completed', 200


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
