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

        stage('Build Release') {
            steps {
                sh '''
                    rustc --version
                    cargo build --release
                '''
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
