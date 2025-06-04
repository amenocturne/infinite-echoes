import { Blockchain, SandboxContract, TreasuryContract } from '@ton/sandbox';
import { Address, beginCell, Cell, toNano, TupleItemInt } from '@ton/core';
import '@ton/test-utils';

import { EchoRegistry } from '../build/EchoRegistry/EchoRegistry_EchoRegistry';

describe('EchoRegistry', () => {
    let blockchain: Blockchain;
    let deployer: SandboxContract<TreasuryContract>;
    let registry: SandboxContract<EchoRegistry>;
    let vaultCode: Cell;
    let pieceCode: Cell;

    beforeEach(async () => {
        blockchain = await Blockchain.create();
        deployer = await blockchain.treasury('deployer');

        vaultCode = beginCell().endCell();
        pieceCode = beginCell().endCell();
        registry = blockchain.openContract(
            await EchoRegistry.fromInit(
                vaultCode,
                pieceCode
            )
        );
        await registry.send(deployer.getSender(), { value: toNano('0.1') }, null);
    });

    it('should deploy', async () => {
        expect(registry.address).toBeDefined();
        const contract = await blockchain.getContract(registry.address);
        expect(contract).toBeDefined();
    });

    it('should create vault and piece on first createPiece', async () => {
        const user = await blockchain.treasury('user1');
        const pieceData = beginCell().storeUint(123, 256).endCell();

        const result = await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData,
                remixedFrom: null,
            },
        );

        // Check registry transaction success
        expect(result.transactions).toHaveTransaction({
            from: user.address,
            to: registry.address,
            success: true,
        });

        // Verify vault was created
        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(Address.isAddress(vaultAddress)).toBe(true);

        // Verify piece was created and added to vault
        const vaultContract = await blockchain.getContract(vaultAddress!);
        const vaultState = await vaultContract.get('getPieceCount');
        expect((vaultState.stack[0] as TupleItemInt).value).toBeGreaterThan(0n);
    });

    it('should reuse existing vault for subsequent pieces', async () => {
        const user = await blockchain.treasury('user2');
        const pieceData1 = beginCell().storeUint(111, 256).endCell();
        const pieceData2 = beginCell().storeUint(222, 256).endCell();

        // First piece creates vault
        await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData1,
                remixedFrom: null,
            },
        );

        const firstVaultAddress = await registry.getGetVaultAddress(user.address);

        // Second piece should reuse vault
        const result = await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData2,
                remixedFrom: null,
            },
        );

        // Verify same vault address
        const secondVaultAddress = await registry.getGetVaultAddress(user.address);
        expect(secondVaultAddress!.toString()).toEqual(firstVaultAddress!.toString());

        // Verify vault has 2 pieces
        const vaultContract = await blockchain.getContract(secondVaultAddress!);
        const vaultState = await vaultContract.get('getPieceCount');
        expect((vaultState.stack[0] as TupleItemInt).value).toBeGreaterThan(1n);
    });

    it('should not return vault for users without pieces', async () => {
        const user = await blockchain.treasury('user3');
        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(vaultAddress).toBeNull();
    });

    describe('Additional User Interactions', () => {
        it('should create piece with remix', async () => {
            const user = await blockchain.treasury('remixUser');
            const originalPiece = await blockchain.treasury('originalPiece');

            // Create original piece
            // Create unique piece data for each test run
            const pieceData = beginCell().storeUint(Date.now(), 256).endCell();
            await registry.send(
                originalPiece.getSender(),
                { value: toNano('0.2') },
                {
                    $$type: 'CreatePiece',
                    pieceData: pieceData,
                    remixedFrom: null,
                },
            );

            // Create remix
            const remixData = beginCell().storeUint(456, 256).endCell();
            const result = await registry.send(
                user.getSender(),
                { value: toNano('0.2') },
                {
                    $$type: 'CreatePiece',
                    pieceData: remixData,
                    remixedFrom: originalPiece.address,
                },
            );

            expect(result.transactions).toHaveTransaction({
                from: user.address,
                to: registry.address,
                success: true,
            });
        });

        it('should fail to create piece with insufficient anti-spam fee', async () => {
            const user = await blockchain.treasury('feeUser');
            const pieceData = beginCell().storeUint(999, 256).endCell();

            const result = await registry.send(
                user.getSender(),
                { value: toNano('0.004') }, // Below minActionFee of 0.005
                {
                    $$type: 'CreatePiece',
                    pieceData: pieceData,
                    remixedFrom: null,
                },
            );

            expect(result.transactions).toHaveTransaction({
                from: user.address,
                to: registry.address,
                success: false,
            });
        });

        it('should handle multiple users creating pieces concurrently', async () => {
            const users = await Promise.all([
                blockchain.treasury('concurrent1'),
                blockchain.treasury('concurrent2'),
                blockchain.treasury('concurrent3'),
            ]);

            // Process users sequentially to avoid race conditions in test environment
            const results = [];
            for (const user of users) {
                const result = await registry.send(
                    user.getSender(),
                    { value: toNano('0.2') },
                    {
                        $$type: 'CreatePiece',
                        pieceData: beginCell().storeUint(Math.floor(Math.random() * 1000), 256).endCell(),
                        remixedFrom: null,
                    },
                );
                results.push(result);
            }

            // Verify all transactions succeeded
            for (const [index, result] of results.entries()) {
                expect(result.transactions).toHaveTransaction({
                    from: users[index].address,
                    to: registry.address,
                    success: true,
                });
            }

            // Verify each user has their own vault
            const vaultAddresses = await Promise.all(users.map(user =>
                registry.getGetVaultAddress(user.address)
            ));

            const uniqueVaults = new Set(vaultAddresses.map(addr => addr?.toString()));
            expect(uniqueVaults.size).toBe(users.length);
        });

    });

    describe('Configuration Management', () => {
        it('should upgrade vault code (owner only)', async () => {
            const newVaultCode = beginCell().endCell();
            const result = await registry.send(
                deployer.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpgradeVaultCode', code: newVaultCode }
            );
            expect(result.transactions).toHaveTransaction({
                from: deployer.address,
                to: registry.address,
                success: true,
            });
        });

        it('should prevent non-owner from upgrading vault code', async () => {
            const attacker = await blockchain.treasury('attacker');
            const newVaultCode = beginCell().endCell();
            const result = await registry.send(
                attacker.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpgradeVaultCode', code: newVaultCode }
            );
            expect(result.transactions).toHaveTransaction({
                from: attacker.address,
                to: registry.address,
                success: false,
            });
        });

        it('should upgrade piece code (owner only)', async () => {
            const newPieceCode = beginCell().endCell();
            const result = await registry.send(
                deployer.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpgradePieceCode', code: newPieceCode }
            );
            expect(result.transactions).toHaveTransaction({
                from: deployer.address,
                to: registry.address,
                success: true,
            });
        });

        it('should prevent non-owner from upgrading piece code', async () => {
            const attacker = await blockchain.treasury('attacker');
            const newPieceCode = beginCell().endCell();
            const result = await registry.send(
                attacker.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpgradePieceCode', code: newPieceCode }
            );
            expect(result.transactions).toHaveTransaction({
                from: attacker.address,
                to: registry.address,
                success: false,
            });
        });

        it('should update fee parameters (owner only)', async () => {
            const newFees = {
                deployValue: toNano('0.03'),
                messageValue: toNano('0.006')
            };
            const result = await registry.send(
                deployer.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpdateFeeParams', ...newFees }
            );
            expect(result.transactions).toHaveTransaction({
                from: deployer.address,
                to: registry.address,
                success: true,
            });
        });

        it('should prevent non-owner from updating fee parameters', async () => {
            const attacker = await blockchain.treasury('attacker');
            const newFees = {
                deployValue: toNano('0.03'),
                messageValue: toNano('0.006')
            };
            const result = await registry.send(
                attacker.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpdateFeeParams', ...newFees }
            );
            expect(result.transactions).toHaveTransaction({
                from: attacker.address,
                to: registry.address,
                success: false,
            });
        });

        it('should update security parameters (owner only)', async () => {
            const newParams = {
                minActionFee: toNano('0.006'),
                coolDownSeconds: 60n
            };
            const result = await registry.send(
                deployer.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpdateSecurityParams', ...newParams }
            );
            expect(result.transactions).toHaveTransaction({
                from: deployer.address,
                to: registry.address,
                success: true,
            });
        });

        it('should prevent non-owner from updating security parameters', async () => {
            const attacker = await blockchain.treasury('attacker');
            const newParams = {
                minActionFee: toNano('0.006'),
                coolDownSeconds: 60n
            };
            const result = await registry.send(
                attacker.getSender(),
                { value: toNano('0.1') },
                { $$type: 'UpdateSecurityParams', ...newParams }
            );
            expect(result.transactions).toHaveTransaction({
                from: attacker.address,
                to: registry.address,
                success: false,
            });
        });
    });
});
