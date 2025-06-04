// TON API Client for interacting with TON blockchain
const TON_TESTNET_API = 'https://testnet.toncenter.com/api/v2/jsonRPC';
const REGISTRY_ADDRESS = 'kQAlmlGXp3ElXKyeLSEXnhacMq117VjqOuzN9r8AJPVEpchv';

// Function to call contract getter methods
async function callContractGetter(address, method, stack = []) {
    try {
        const response = await fetch(TON_TESTNET_API, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: '2.0',
                method: 'runGetMethod',
                params: {
                    address: address,
                    method: method,
                    stack: stack
                }
            })
        });

        const data = await response.json();
        
        if (data.error) {
            console.error('TON API error:', data.error);
            return null;
        }
        
        return data.result;
    } catch (error) {
        console.error('Error calling TON contract:', error);
        return null;
    }
}

// Get the current piece version from the registry
async function getPieceVersion() {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getPieceVersion');
    if (result && result.exit_code === 0 && result.stack.length > 0) {
        // Parse the result - TON returns [["num",value]] format
        return parseInt(result.stack[0][1], 10);
    }
    return null;
}

// Get the current vault version from the registry
async function getVaultVersion() {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getVaultVersion');
    if (result && result.exit_code === 0 && result.stack.length > 0) {
        return parseInt(result.stack[0][1], 10);
    }
    return null;
}

// Get fee parameters from the registry
async function getFeeParams() {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getFeeParams');
    if (result && result.exit_code === 0 && result.stack.length >= 2) {
        return {
            deployValue: result.stack[0][1],
            messageValue: result.stack[1][1]
        };
    }
    return null;
}

// Get security parameters from the registry
async function getSecurityParams() {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getSecurityParams');
    if (result && result.exit_code === 0 && result.stack.length >= 2) {
        return {
            minActionFee: result.stack[0][1],
            coolDownSeconds: parseInt(result.stack[1][1], 10)
        };
    }
    return null;
}

// Export the functions
window.tonApi = {
    getPieceVersion,
    getVaultVersion,
    getFeeParams,
    getSecurityParams
};
