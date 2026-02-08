pipeline {
    agent any

    environment {
        CARGO_HOME   = "${WORKSPACE}/.cargo"
        RUSTUP_HOME  = "${env.HOME}/.rustup"
        PATH         = "${env.HOME}/.cargo/bin:${env.PATH}"
        REPO         = 'rileyseaburg/codetether-agent'
        BINARY_NAME  = 'codetether'
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

        stage('Toolchain') {
            steps {
                sh '''
                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Lint') {
            steps {
                sh 'cargo clippy --all-features -- -D warnings 2>&1 || true'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test 2>&1'
            }
        }

        stage('Build Release') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Package') {
            when {
                buildingTag()
            }
            steps {
                script {
                    env.VERSION = env.TAG_NAME ?: sh(script: "grep '^version' Cargo.toml | head -1 | sed 's/.*\"\\(.*\\)\"/\\1/'", returnStdout: true).trim()
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
            }
        }

        stage('GitHub Release') {
            when {
                buildingTag()
            }
            steps {
                withCredentials([string(credentialsId: 'github-token', variable: 'GH_TOKEN')]) {
                    sh """
                        # Check if release already exists
                        if gh release view ${env.TAG_NAME} --repo ${REPO} > /dev/null 2>&1; then
                            echo "Release ${env.TAG_NAME} exists, uploading artifacts..."
                            gh release upload ${env.TAG_NAME} \
                                dist/${env.ARTIFACT}.tar.gz \
                                dist/${env.ARTIFACT} \
                                dist/SHA256SUMS-${env.VERSION}.txt \
                                --repo ${REPO} --clobber
                        else
                            echo "Creating release ${env.TAG_NAME}..."
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

        stage('Deploy to Server') {
            when {
                buildingTag()
            }
            steps {
                withCredentials([sshUserPrivateKey(credentialsId: 'deploy-ssh-key', keyFileVariable: 'SSH_KEY', usernameVariable: 'SSH_USER')]) {
                    sh """
                        scp -o StrictHostKeyChecking=no -i \$SSH_KEY \
                            target/release/${BINARY_NAME} \
                            \$SSH_USER@192.168.50.133:/usr/local/bin/${BINARY_NAME}
                    """
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
        cleanup {
            cleanWs(deleteDirs: true, patterns: [[pattern: '.cargo/**', type: 'INCLUDE']])
        }
    }
}
