export interface BlockHeader {
    version: Buffer;
    prevHash: Buffer;
    merkleRoot: Buffer;
    time: Buffer;
    bits: Buffer;
    nonce: Buffer;
}

export class BitcoinUtils {
    /**
     * 将原始Buffer转换为结构化的区块头
     */
    static createBlockHeader(data: Buffer): BlockHeader {
        return {
            version: data.subarray(0, 4),
            prevHash: data.subarray(4, 36),
            merkleRoot: data.subarray(36, 68),
            time: data.subarray(68, 72),
            bits: data.subarray(72, 76),
            nonce: data.subarray(76, 80),
        };
    }

    /**
     * 将结构化的区块头转换回Buffer
     */
    static blockHeaderToBuffer(header: BlockHeader): Buffer {
        return Buffer.concat([
            header.version,
            header.prevHash,
            header.merkleRoot,
            header.time,
            header.bits,
            header.nonce,
        ]);
    }

    /**
     * 将时间戳Buffer转换为数字
     */
    static timeBufferToNumber(timeBuffer: Buffer): number {
        return timeBuffer.readUInt32LE(0);
    }
}