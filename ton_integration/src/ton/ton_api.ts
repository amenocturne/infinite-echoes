const TON_TESTNET_API = 'https://testnet.toncenter.com/api/v2/jsonRPC';
const REGISTRY_ADDRESS = 'kQAlmlGXp3ElXKyeLSEXnhacMq117VjqOuzN9r8AJPVEpchv';

interface ContractInfo {
    pieceVersion: number | null;
    vaultVersion: number | null;
    feeParams: { deployValue: string; messageValue: string } | null;
    securityParams: { minActionFee: string; coolDownSeconds: number } | null;
}

async function callContractGetter(address: string, method: string, stack: any[] = []): Promise<any | null> {
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

export async function getPieceVersion(): Promise<number | null> {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getPieceVersion');
    if (result && result.exit_code === 0 && result.stack.length > 0) {
        return parseInt(result.stack[0][1], 10);
    }
    return null;
}

export async function getVaultVersion(): Promise<number | null> {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getVaultVersion');
    if (result && result.exit_code === 0 && result.stack.length > 0) {
        return parseInt(result.stack[0][1], 10);
    }
    return null;
}

export async function getFeeParams(): Promise<{ deployValue: string; messageValue: string } | null> {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getFeeParams');
    if (result && result.exit_code === 0 && result.stack.length >= 2) {
        return {
            deployValue: result.stack[0][1],
            messageValue: result.stack[1][1]
        };
    }
    return null;
}

export async function getSecurityParams(): Promise<{ minActionFee: string; coolDownSeconds: number } | null> {
    const result = await callContractGetter(REGISTRY_ADDRESS, 'getSecurityParams');
    if (result && result.exit_code === 0 && result.stack.length >= 2) {
        return {
            minActionFee: result.stack[0][1],
            coolDownSeconds: parseInt(result.stack[1][1], 10)
        };
    }
    return null;
}

export let contractInfo: ContractInfo = {
    pieceVersion: null,
    vaultVersion: null,
    feeParams: null,
    securityParams: null
};

export async function fetchContractInfo(): Promise<ContractInfo | null> {
    try {
        const pieceVersion = await getPieceVersion();
        const vaultVersion = await getVaultVersion();
        const feeParams = await getFeeParams();
        const securityParams = await getSecurityParams();

        contractInfo = {
            pieceVersion,
            vaultVersion,
            feeParams,
            securityParams
        };

        updateContractInfoDisplay();

        console.log("Contract info loaded:", contractInfo);
        return contractInfo;
    } catch (error) {
        console.error("Error fetching contract info:", error);
        setTimeout(fetchContractInfo, 3000);
        return null;
    }
}

export function updateContractInfoDisplay(): void {
    const contractInfoElement = document.getElementById('contract-info');
    if (!contractInfoElement) return;

    if (contractInfo && contractInfo.pieceVersion !== null) {
        contractInfoElement.innerHTML = `
            <div>Piece Version: ${contractInfo.pieceVersion}</div>
            ${contractInfo.vaultVersion !== null ? `<div>Vault Version: ${contractInfo.vaultVersion}</div>` : ''}
        `;
    } else {
        contractInfoElement.innerHTML = '<div>Loading contract info...</div>';
    }
}
