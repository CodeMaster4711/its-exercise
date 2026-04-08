package de.fhro.inf.its.exercise;

import java.io.FileInputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.security.*;
import java.security.cert.Certificate;
import java.security.cert.CertificateException;
import java.security.cert.CertificateFactory;
import java.security.interfaces.RSAPrivateKey;
import java.security.spec.InvalidKeySpecException;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.X509EncodedKeySpec;
import java.util.Base64;


public class CryptoUtil {
    /**
     * read Base64 encoded Certificate from a file in format PEM
     *
     * @param certificateFile filename of certificate file
     */
    public static Certificate readCertificate(String certificateFile) throws CertificateException, IOException {
        try (FileInputStream fis = new FileInputStream(certificateFile)) {
            CertificateFactory cf = CertificateFactory.getInstance("X.509");
            return cf.generateCertificate(fis);
        }
    }

    /**
     * read Base64 encoded public key from file in format PEM
     *
     * @param file name of file containing public key
     * @return public key
     */
    public static PublicKey getPublicKey(String file) throws IOException, NoSuchAlgorithmException, InvalidKeySpecException {
        String publicKeyPEM = new String(Files.readAllBytes(Paths.get(file)));

        // Remove the first and last lines
        String pubKeyPEM = publicKeyPEM
                .replace("-----BEGIN PUBLIC KEY-----\n", "")
                .replace("-----END PUBLIC KEY-----", "")
                .replaceAll("\\s", ""); // Remove all whitespace

        // Base64 decode the data
        byte[] encoded = Base64.getDecoder().decode(pubKeyPEM);

        X509EncodedKeySpec keySpec = new X509EncodedKeySpec(encoded);
        KeyFactory kf = KeyFactory.getInstance("RSA");
        return kf.generatePublic(keySpec);
    }

    /**
     * read Base64 encoded private key from file in format PEM
     *
     * @param filename filename of private key file
     */
    public static PrivateKey getPrivateKey(String filename) throws IOException, NoSuchAlgorithmException, InvalidKeySpecException {
        String privateKeyPEM = Files.readString(Path.of(filename));

        // strip of header, footer, newlines, whitespaces
        privateKeyPEM = privateKeyPEM
                .replace("-----BEGIN PRIVATE KEY-----", "")
                .replaceAll(System.lineSeparator(), "")
                .replace("-----END PRIVATE KEY-----", "")
                .replaceAll("\\s", "");

        // decode to get the binary DER representation
        byte[] privateKeyDER = Base64.getDecoder().decode(privateKeyPEM);

        // Decode the key which is stored in PKCS#8 format
        PKCS8EncodedKeySpec keySpec = new PKCS8EncodedKeySpec(privateKeyDER);
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        return (RSAPrivateKey) keyFactory.generatePrivate(keySpec);
    }


    /**
     * generate a key pair for RSA an store the two keys in files
     * @param length  key length of RSA keys
     * @param privateKeyFile filename for private key
     * @param publicKeyFile filename for public key
     */
    public static void generateRsaKeyPair(int length, String privateKeyFile, String publicKeyFile) throws NoSuchAlgorithmException, IOException {
        // Generate the RSA Key Pair
        KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
        keyGen.initialize(length);
        KeyPair pair = keyGen.generateKeyPair();

        // Save the Private Key in PEM format
        String privateKeyContent = "-----BEGIN PRIVATE KEY-----\n";
        // Base64 Mime Encoder inserts line breaks and is better suited for PEM format
        privateKeyContent += Base64.getMimeEncoder().encodeToString(pair.getPrivate().getEncoded());
        privateKeyContent += "\n-----END PRIVATE KEY-----";

        FileWriter writer = new FileWriter(privateKeyFile);
        writer.write(privateKeyContent);
        writer.close();

        // Save the Public Key in PEM format
        String publicKeyContent = "-----BEGIN PUBLIC KEY-----\n";
        publicKeyContent += Base64.getMimeEncoder().encodeToString(pair.getPublic().getEncoded());
        publicKeyContent += "\n-----END PUBLIC KEY-----";

        FileWriter writer2 = new FileWriter(publicKeyFile);
        writer2.write(publicKeyContent);
        writer2.close();
    }
}
