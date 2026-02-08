pipeline {
    agent any

    environment {
        CARGO_HOME            = "${WORKSPACE}/.cargo"
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
                        cargo build --release
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
                        if gh release view ${env.TAG_NAME} --repo ${REPO} > /dev/null 2>&1; then
                            gh release upload ${env.TAG_NAME} \
                                dist/${env.ARTIFACT}.tar.gz \
                                dist/${env.ARTIFACT} \
                                dist/SHA256SUMS-${env.VERSION}.txt \
                                --repo ${REPO} --clobber
                        else
                            gh release create ${env.TAG_NAME} \
                                dist/${env.ARTIFACT}.tar.gz \
                                dist/${env.ARTIFACT} \
                                dist/SHA256SUMS-${env.VERSION}.txt \
                                --repo ${REPO} \
                                --title "${env.TAG_NAME} - CodeTether Agent" \
                                --generate-notes
                        fi
                    """
                }
            }
        }

        stage('Publish to crates.io') {
            when {
                buildingTag()
            }
            steps {
                withCredentials([string(credentialsId: 'cargo-registry-token', variable: 'CARGO_REGISTRY_TOKEN')]) {
                    sh '''
                        echo "Publishing v${TAG_NAME} to crates.io ..."
                        cargo publish --locked
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
