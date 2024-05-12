from sessionless import SessionlessSecp256k1
import requests

async def do_cool_stuff(registration_tuple):
    print('registered!')
    print('Gonna just do blue for now')

    uuid = registration_tuple[0].uuid
    private_key = registration_tuple[1]
    public_key = registration_tuple[2]
    
    sessionless = SessionlessSecp256k1()

    timestamp = 'right now' #we'll deal with time later

    def get_key():
        return private_key

    signature = await sessionless.sign(f'{{"uuid":"{uuid}","coolness":"max","timestamp":"{timestamp}"}}', get_key)

    message = {
        "uuid": uuid,
        "coolness": "max",
        "timestamp": timestamp,
        "signature": signature
    }

    response = requests.post('http://127.0.0.1:3001/cool-stuff', json=message) # update this to be dynamic
    
    print(f'response: {response}')

    return response