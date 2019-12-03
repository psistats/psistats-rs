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
}