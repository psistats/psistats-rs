pipeline {
  agent {
    label 'master'
  }
  stages {
    stage('Prepare') {
      steps {
        sh 'cargo clean'
        sh 'cargo install cargo-deb || true'
        sh 'cargo install cargo-rpm || true'
        sh 'cargo install cargo-config || true'
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build --bin psistats --release'

      }
    }
    stage('Publish') {
      steps {
        sh 'build/linux.sh'
        archiveArtifacts artifacts: 'target/release/psistats', onlyIfSuccessful: true
        archiveArtifacts artifacts: 'target/debian/*.deb', onlyIfSuccessful: true
      }
    }
  }
  post {
    success {
      emailext (
        subject: "JOB: ${env.JOB_NAME} [${env.BUILD_NUMBER}] - Status: SUCCESSFUL",
        body: """${env.JOB_NAME} [${env.BUILD_NUMBER}] was completed successfully.

Check console output at ${env.BUILD_URL}

___  ____ _ _  _ ____ _  _
|__] [__  | |_/  |  | |\\ |
|    ___] | | \\_ |__| | \\|

        """,
        to: "ci@psikon.com"
      )
    }
    failure {
      emailext (
        subject: "JOB: ${env.JOB_NAME} [${env.BUILD_NUMBER}] - Status: FAILURE",
        body: """
${env.JOB_NAME} [${env.BUILD_NUMBER}] has failed!

Check full console output at ${env.BUILD_URL}

___  ____ _ _  _ ____ _  _
|__] [__  | |_/  |  | |\\ |
|    ___] | | \\_ |__| | \\|

      """,
        to: "ci@psikon.com"
      )
    }
  }
}