pipeline {
  agent {
    label 'master'
  }
  stages {
    stage('Build') {
      steps {
        sh 'build/linux.sh'
      }
    }
  }
  post {
    success {
      emailext (
        subject: "JOB: ${env.JOB_NAME} [${env.BUILD_NUMBER}] - Status: SUCCESSFUL",
        body: '''${env.JOB_NAME} [${env.BUILD_NUMBER}] was completed successfully.

Check console output at ${env.BUILD_URL}

___  ____ _ _  _ ____ _  _
|__] [__  | |_/  |  | |\\ |
|    ___] | | \\_ |__| | \\|

        ''',
        to: "ci@psikon.com"
      )
    }
    failure {
      emailext (
        subject: "JOB: ${env.JOB_NAME} [${env.BUILD_NUMBER}] - Status: FAILURE",
        body: """${env.JOB_NAME} [${env.BUILD_NUMBER}] has failed! \

Check console output at ${env.BUILD_URL}

___  ____ _ _  _ ____ _  _
|__] [__  | |_/  |  | |\\ |
|    ___] | | \\_ |__| | \\|

      """,
        to: "ci@psikon.com"
      )
    }
  }
}