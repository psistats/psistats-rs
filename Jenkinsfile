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

@Library('jenkins-pipeline-appveyor@master') _
@Library('psikon-jenkins-mailer@master') _

pipeline {

  agent {
    node {
      label 'master'
    }
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
            stage('Build Raspberry Pi') {
              steps {
                sh 'build/linux/build.sh armv7-unknown-linux-gnueabihf'
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
  }
  post {
    success {
      updateGithubCommitStatus currentBuild
      psikonMailer currentBuild env
    }

    failure {
      updateGithubCommitStatus currentBuild
      psikonMailer currentBuild env
    }
  }
}
    /*
    success {
      updateGithubCommitStatus(currentBuild)
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
      updateGithubCommitStatus(currentBuild)
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


Main Controls - *FIGlet and AOL Macro Fonts Supported*
Font:
Character Width:
Character Height:
Type Something


Other Stuff From patorjk.com That You Might Like:

    Typing Speed Test
    Keyboard Layout Analzyer
    Text Color Fader
    Snake Game
    My Photography Site
    Main Page

patorjk.com

  _______   _______   ___   ___ ___    _______   ______
 |   _   | |   _   | |   | |   Y   )  |   _   | |   _  \
 |.  1   | |   1___| |.  | |.  1  /   |.  |   | |.  |   |
 |.  ____| |____   | |.  | |.  _  \   |.  |   | |.  |   |
 |:  |     |:  1   | |:  | |:  |   \  |:  1   | |:  |   |
 |::.|     |::.. . | |::.| |::.| .  ) |::.. . | |::.|   |
 `---'     `-------' `---' `--- ---'  `-------' `--- ---'

                  https://psikon.org

*/