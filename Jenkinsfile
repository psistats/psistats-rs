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
      label 'controller'
    }
  }

  libraries {
    lib('psikon-jenkins-appveyor@master')
    lib('psikon-jenkins-mailer@master')
  }

  stages {
    stage('Preface') {
      steps {
        githubNotify context: 'ci/jenkins',
                      credentialsId: 'psikon-ci-github-usertoken',
                      description: 'Waiting for Jenkins tasks to complete',
                      status: 'PENDING',
                      targetUrl: 'https://dev.psikon.org/jenkins'
      }
    }
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
                githubNotify context: 'ci/jenkins/x86_64',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'Building x86_64 linux binaries',
                             status: 'PENDING',
                             targetUrl: 'https://dev.psikon.org/jenkins'
                sh 'build/linux/build.sh x86_64-unknown-linux-gnu'
                githubNotify context: 'ci/jenkins/x86_64',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'x86_64 linux binaries built',
                             status: 'SUCCESS',
                             targetUrl: 'https://dev.psikon.org/jenkins'
              }
            }
            stage('Build ArmV7') {
              steps {
                githubNotify context: 'ci/jenkins/armv7',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'Building 32bit arm linux binaries',
                             status: 'PENDING',
                             targetUrl: 'https://dev.psikon.org/jenkins'
                sh 'build/linux/build.sh armv7-unknown-linux-gnueabihf'
                githubNotify context: 'ci/jenkins/armv7',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: '32bit arm linux binaries built',
                             status: 'SUCCESS',
                             targetUrl: 'https://dev.psikon.org/jenkins'
              }
            }
            stage('Build Arm64') {
              steps {
                githubNotify context: 'ci/jenkins/arm64',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'Building 64bit arm linux binaries',
                             status: 'PENDING',
                             targetUrl: 'https://dev.psikon.org/jenkins'
                sh 'build/linux/build.sh aarch64-unknown-linux-gnu'
                githubNotify context: 'ci/jenkins/arm64',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: '64bit arm linux binaries built',
                             status: 'SUCCESS',
                             targetUrl: 'https://dev.psikon.org/jenkins'
              }
            }
          }
        }
        stage('Windows') {

          stages {
            stage('Build Windows') {
              steps {
                githubNotify context: 'ci/jenkins/windows',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'Building Windows binaries',
                             status: 'PENDING',
                             targetUrl: 'https://dev.psikon.org/jenkins'
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
                githubNotify context: 'ci/jenkins/windows',
                             credentialsId: 'psikon-ci-github-usertoken',
                             description: 'Windows artifacts built',
                             status: 'SUCCESS',
                             targetUrl: 'https://dev.psikon.org/jenkins'
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
    stage('Deploy BETA') {
      when { branch "develop" }
      steps {
        sh 'build/linux/deploy-debian.sh testing'
      }
    }
    stage('Deploy') {
      when { branch "master" }
      steps {
        sh 'build/linux/deploy-debian.sh main'
      }
    }

  }
  post {
    success {
      githubNotify context: 'ci/jenkins',
                    credentialsId: 'psikon-ci-github-usertoken',
                    description: 'All Jenkins tasks completed',
                    status: 'SUCCESS',
                    targetUrl: 'https://dev.psikon.org/jenkins'
      psikonMailer(currentBuild, env)
    }

    failure {
      githubNotify context: 'ci/jenkins',
                    credentialsId: 'psikon-ci-github-usertoken',
                    description: 'One or more tasks have failed',
                    status: 'FAILURE',
                    targetUrl: 'https://dev.psikon.org/jenkins'
      psikonMailer(currentBuild, env)
    }
  }
}
