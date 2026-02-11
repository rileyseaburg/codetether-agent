// Jenkins Job Configuration Requirements:
// - Multibranch Pipeline: Add "Discover tags" behavior in Branch Sources → Git → Behaviors
// - Or configure GitHub webhook to trigger builds on tag push events
// - The "Package & Release" and "Publish to crates.io" stages only run when building tags

pipeline {
    agent any

    environment {
        CARGO_HOME            = "${env.HOME}/.cargo"
        RUSTUP_HOME           = "${env.HOME}/.rustup"
        PATH                  = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"
        REPO                  = 'rileyseaburg/codetether-agent'
        BINARY_NAME           = 'codetether'

        // sccache backed by MinIO S3
        RUSTC_WRAPPER         = 'sccache'
        SCCACHE_BUCKET        = 'sccache'
        SCCACHE_ENDPOINT      = 'http://192.168.50.223:9000'
        SCCACHE_REGION        = 'us-east-1'
        SCCACHE_S3_KEY_PREFIX = 'codetether'
        SCCACHE_S3_USE_SSL    = 'false'
    }

    options {
        timestamps()
        timeout(time: 30, unit: 'MINUTES')
        disableConcurrentBuilds()
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
                // Clean stale .cargo from workspace (CARGO_HOME is now $HOME/.cargo)
                sh 'rm -rf "${WORKSPACE}/.cargo"'
            }
        }

        stage('Build Release') {
            steps {
                withCredentials([
                    string(credentialsId: 'minio-access-key', variable: 'AWS_ACCESS_KEY_ID'),
                    string(credentialsId: 'minio-secret-key', variable: 'AWS_SECRET_ACCESS_KEY')
                ]) {
                    sh '''
                        rustc --version
                        sccache --start-server 2>/dev/null || true
                        sccache --show-stats 2>&1 | grep "Cache location" || true
                        cargo build --release --features functiongemma
                        echo "=== sccache stats ==="
                        sccache --show-stats || true
                    '''
                }
            }
        }

        stage('Package & Release') {
            when {
                buildingTag()
            }
            steps {
                script {
                    env.VERSION = env.TAG_NAME
                    env.PLATFORM = 'x86_64-unknown-linux-gnu'
                    env.ARTIFACT = "${BINARY_NAME}-${VERSION}-${PLATFORM}"
                }
                sh """
                    mkdir -p dist
                    cp target/release/${BINARY_NAME} dist/${env.ARTIFACT}
                    cd dist && tar czf ${env.ARTIFACT}.tar.gz ${env.ARTIFACT}
                    sha256sum ${env.ARTIFACT}.tar.gz ${env.ARTIFACT} > SHA256SUMS-${env.VERSION}.txt
                """
                archiveArtifacts artifacts: "dist/${env.ARTIFACT}.tar.gz, dist/SHA256SUMS-${env.VERSION}.txt", fingerprint: true

                withCredentials([string(credentialsId: 'github-token', variable: 'GH_TOKEN')]) {
                    sh """
                        gh release create ${env.TAG_NAME} \
                            dist/${env.ARTIFACT}.tar.gz \
                            dist/${env.ARTIFACT} \
                            dist/SHA256SUMS-${env.VERSION}.txt \
                            --repo ${REPO} \
                            --title "${env.TAG_NAME} - CodeTether Agent" \
                            --generate-notes \
                            --latest \
                            --verify-tag || \
                        gh release upload ${env.TAG_NAME} \
                            dist/${env.ARTIFACT}.tar.gz \
                            dist/${env.ARTIFACT} \
                            dist/SHA256SUMS-${env.VERSION}.txt \
                            --repo ${REPO} --clobber
                    """
                }
            }
        }

        stage('Publish to crates.io') {
            when {
                buildingTag()
            }
            steps {
                withCredentials([
                    string(credentialsId: 'cargo-registry-token', variable: 'CARGO_REGISTRY_TOKEN'),
                    string(credentialsId: 'minio-access-key', variable: 'AWS_ACCESS_KEY_ID'),
                    string(credentialsId: 'minio-secret-key', variable: 'AWS_SECRET_ACCESS_KEY')
                ]) {
                    sh '''
                        echo "Publishing ${TAG_NAME} to crates.io ..."
                        cargo publish --allow-dirty --features functiongemma
                    '''
                }
            }
        }
    }

    post {
        success {
            echo "Build successful: ${env.TAG_NAME ?: env.BRANCH_NAME}"
        }
        failure {
            echo "Build failed: ${env.TAG_NAME ?: env.BRANCH_NAME}"
        }
    }
}
