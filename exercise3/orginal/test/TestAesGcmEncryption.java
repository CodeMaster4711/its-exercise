package de.fhro.inf.its.exercise3.test;

import de.fhro.inf.its.uebung3.DecryptAesGcm;
import de.fhro.inf.its.uebung3.EncryptAesGcm;
import de.fhro.inf.its.uebung3.utils.CryptoUtils;
import de.fhro.inf.its.uebung3.utils.FileUtils;
import de.fhro.inf.its.uebung3.KeyStoreUtils;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import javax.crypto.AEADBadTagException;
import javax.crypto.SecretKey;
import java.util.logging.Logger;

import static org.junit.jupiter.api.Assertions.fail;

public class TestAesGcmEncryption {
    private static final String FILENAME = "test.txt";
    private static final String ENCRYPTED_FILE = "encryptedTest.bin";
    private static final String KEYSTORE_FILE = "keystore.dat";
    private static final String KEY_ALIAS = "PrivateKey";

    // Passwords for Keystore and Key
    // ATTENTION: in real applications DO NOT WRITE PASSWORDS OR SECRETS IN SOURCE CODE,
    // they must be passed to the application in a secure way
    private static final char[] KEY_PASSWORD = "Password".toCharArray();
    private static final char[] KEYSTORE_PASSWORD = "test".toCharArray();

    private static final Logger logger = Logger.getLogger(TestAesGcmEncryption.class.getName());

    @Test
    public void testEncryption() {
        createAndFillKeystore();

        encryption();
        decryption();
    }

    /**
     * Generate an AES key and store it in a Keystore
     */
    private void createAndFillKeystore() {
        try {
            var keyStore = new KeyStoreUtils(KEYSTORE_PASSWORD);
            SecretKey key = CryptoUtils.generateAESKey();
            keyStore.addKey(key, KEY_ALIAS, KEY_PASSWORD);
            keyStore.writeKeyStoreToFile(KEYSTORE_FILE, KEYSTORE_PASSWORD);
        } catch (Exception e) {
            logger.severe("Error when creating KeyStore");
            logger.severe(e.toString());
            fail();
        }
    }

    /**
     * Encrypt a file with AES GCM
     */
    private void encryption() {
        try {
            var keyStore = new KeyStoreUtils(KEYSTORE_FILE, KEYSTORE_PASSWORD);
            SecretKey key = keyStore.getKey(KEY_ALIAS, KEY_PASSWORD);
            byte[] data = FileUtils.readFromFile(FILENAME);

            // Authentication data must be known at decryption
            // if no data is provided the MAC will be calculated only for the ciphertext
            byte[] aad = "Any Authentication Data".getBytes();
            byte[] encryptedData = EncryptAesGcm.encrypt(data, aad, key);

            FileUtils.writeToFile(ENCRYPTED_FILE, encryptedData);

        } catch (Exception e) {
            logger.severe("Error in encryption");
            logger.severe(e.toString());
            fail();
        }
    }

    /**
     * Decrypt a file with AES GCM
     */
    private void decryption() {
        try {
            var keyStore = new KeyStoreUtils(KEYSTORE_FILE, KEYSTORE_PASSWORD);
            SecretKey key = keyStore.getKey(KEY_ALIAS, KEY_PASSWORD);
            byte[] data = FileUtils.readFromFile(ENCRYPTED_FILE);

            // Authentication data is an information between sender and receiver to check authenticity
            // it can be transferred without enrcyption because its integrity is protected by the MAC
            byte[] aad = "Any Authentication Data".getBytes();
            byte[] decryptedData = DecryptAesGcm.decrypt(data, aad, key);

            logger.info(() -> "Decrypted File: " + new String(decryptedData));
            byte[] fileContent = FileUtils.readFromFile(FILENAME);
            Assertions.assertArrayEquals(decryptedData, fileContent);
            logger.info("Decryption successful");

            // now check if a manipulation of the cipherdata will be detected
            byte[] manipulatedData = FileUtils.readFromFile(ENCRYPTED_FILE);
            try {
                logger.info("manipulation of cipherdata");
                manipulatedData[13] = (byte) 13;
                manipulatedData[14] = (byte) 14;
                byte[] noData = DecryptAesGcm.decrypt(manipulatedData, aad, key);
            } catch (AEADBadTagException e){
                logger.info("Integrity check failed");
                logger.info("manipulation of cipherdata detected");
            }

        } catch (Exception e) {
            logger.severe("Error in decryption");
            logger.severe(e.toString());

            fail();
        }
    }
}
