def getRepoURL() {
  sh "git config --get remote.origin.url > .git/remote-url"
  return readFile(".git/remote-url").trim()
}

def getCommitSha() {
  sh "git rev-parse HEAD > .git/current-commit"
  return readFile(".git/current-commit").trim()
}

def updateGithubCommitStatus(build) {
  // workaround https://issues.jenkins-ci.org/browse/JENKINS-38674
  repoUrl = getRepoURL()
  commitSha = getCommitSha()

  step([
    $class: 'GitHubCommitStatusSetter',
    reposSource: [$class: "ManuallyEnteredRepositorySource", url: repoUrl],
    commitShaSource: [$class: "ManuallyEnteredShaSource", sha: commitSha],
    errorHandlers: [[$class: 'ShallowAnyErrorHandler']],
    statusResultSource: [
      $class: 'ConditionalStatusResultSource',
      results: [
        [$class: 'BetterThanOrEqualBuildResult', result: 'SUCCESS', state: 'SUCCESS', message: build.description],
        [$class: 'BetterThanOrEqualBuildResult', result: 'FAILURE', state: 'FAILURE', message: build.description],
        [$class: 'AnyBuildResult', state: 'FAILURE', message: 'Loophole']
      ]
    ]
  ])
}

pipeline {

  agent {
    node {
      label 'master'
    }
  }

  libraries {
    lib('psikon-jenkins-appveyor@master')
    lib('psikon-jenkins-mailer@master')
  }

  stages {
    stage('Build') {
      parallel {

        stage('Linux') {
          stages {
            stage('Prepare') {
              steps {
                sh 'cargo clean'
              }
            }
            stage('Build x86_64') {
              steps {
                sh 'build/linux/build.sh x86_64-unknown-linux-gnu'
              }
            }
            stage('Build ArmV7') {
              steps {
                sh 'build/linux/build.sh armv7-unknown-linux-gnueabihf'
              }
            }
            stage('Build Arm64') {
              steps {
                sh 'build/linux/build.sh aarch64-unknown-linux-gnu'
              }
            }
          }
        }
        stage('Windows') {

          stages {
            stage('Build Windows') {
              steps {
                withCredentials([string(credentialsId: 'appveyor-token', variable: 'TOKEN')]) {
                  appveyorBuild(
                    accountToken: TOKEN,
                    accountName: 'alex-dow',
                    projectSlug: 'psistats-rs',
                    branch: env.GIT_BRANCH,
                    commitId: env.GIT_COMMIT,
                    buildNumber: env.BUILD_NUMBER
                  )
                }
              }
            }

            stage('Download Appveyor Artifacts') {
              steps {
                script {
                  appveyorDownloadAll(
                    accountName: 'alex-dow',
                    projectSlug: 'psistats-rs',
                    buildVersion: env.APPVEYOR_BUILD_VERSION,
                    targetDir: 'target/release/artifacts'
                  )
                }
              }
            }
          }
        }
      }
    }
    stage('Publish') {
      steps {
        archiveArtifacts artifacts: 'target/release/artifacts/**/*', onlyIfSuccessful: true
      }
    }
    stage('Deploy') {
      when { branch "master" }
      steps {
        sh 'build/linux/deploy-debian.sh testing'
      }
    }
  }
  post {
    success {
      updateGithubCommitStatus(currentBuild)
      psikonMailer(currentBuild, env)
    }

    failure {
      updateGithubCommitStatus currentBuild
      psikonMailer(currentBuild, env)
    }
  }
}
