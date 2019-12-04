def appveyor_download_artifacts(accountName, projectSlug, buildVersion) {

  echo '[APPVEYOR] Downloading artifacts';

  def content = httpRequest(
    url: "https://ci.appveyor.com/api/projects/${accountName}/${projectSlug}/build/${buildVersion}",
    customHeaders: [
      [name: 'Accept', value: 'application/json']
    ]
  );
  echo groovy.json.JsonOutput.prettyPrint(content.getContent());
  def build_obj = new groovy.json.JsonSlurperClassic().parseText(content.getContent());

  def job_id = build_obj.build.jobs[0].jobId;

  def artifact_response = httpRequest(
    url: "https://ci.appveyor.com/api/buildjobs/${job_id}/artifacts",
    customHeaders: [
      [name: 'Accept', value: 'application/json']
    ]
  );

  def artifact_response_content = artifact_response.getContent();
  echo artifact_response_content;

  build_obj = new groovy.json.JsonSlurperClassic().parseText(artifact_response_content);

  build_obj.each {
    echo "[APPVEYOR] Artifact found: ${it.fileName}";
    def f = new File(it.fileName);
    def fn = f.getName();
    def encodedFn = java.net.URLEncoder.encode(it.fileName, 'UTF-8');
    sh(script: """wget -O target/${fn} https://ci.appveyor.com/api/buildjobs/${job_id}.artifacts/${encodedFn}""");
  };
}

def appveyor_start_build(appveyorToken, accountName, projectSlug, branch, commitId) {
  echo '[APPVEYOR] Starting appveyor job';

  def request = [:]
  request['accountName'] = accountName;
  request['projectSlug'] = projectSlug;
  request['environmentVariables'] = [:];
  request['environmentVariables']['JENKINS_BUILD_NUMBER'] = env.BUILD_NUMBER;

  if (branch.startsWith('PR')) {
    echo '[APPVEYOR] Building a pull request';
    def pr = branch.split('-')[1];
    request['pullRequestId'] = pr;
  } else {
    echo "[APPVEYOR] Building ${branch} : ${commitId}";
    request['branch'] = branch;
    request['commitId'] = commitId;
  }

  def requestBody = new groovy.json.JsonBuilder(request).toPrettyString();

  // echo "[APPVEYOR] Request body: ${request_body}";
  def build_response = httpRequest(
    url: "https://ci.appveyor.com/api/account/${accountName}/builds",
    httpMode: 'POST',
    customHeaders: [
      [name: 'Authorization', value: "Bearer ${appveyorToken}"],
      [name: 'Content-type', value: 'application/json']
    ],
    requestBody: requestBody
  )

  def content = build_response.getContent();

  def build_obj = new groovy.json.JsonSlurperClassic().parseText(content)
  echo "[APPVEYOR] Appveyor build number: ${build_obj.buildNumber}";
  echo "[APPVEYOR] Appveyor build version: ${build_obj.version}";

  return build_obj;
}

def appveyor_build_status(appveyorToken, accountName, projectSlug, buildVersion) {
  def status_response = httpRequest(
      url: "https://ci.appveyor.com/api/projects/${accountName}/${projectSlug}/build/${buildVersion}",
      httpMode: 'GET',
      customHeaders: [
          [name: 'Authorization', value: "Bearer ${appveyorToken}"],
          [name: 'Accept', value: 'application/json']
      ]
  )

  def status_content = status_response.getContent()
  def build_data = new groovy.json.JsonSlurperClassic().parseText(status_content)

  return build_data.build.status;

}


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
    label 'master'
  }
  stages {
    stage('Prepare') {
      steps {
        updateGithubCommitStatus(currentBuild)
        sh 'cargo install cargo-deb || true'
        sh 'cargo install cargo-config || true'
      }
    }
    stage('Build') {
      parallel {

        stage('Linux') {
          stages {
            stage('Build x86_64') {
              steps {
                sh 'cargo clean'
                sh 'cargo build --bin psistats --release --verbose --target x86_64-unknown-linux-gnu'
              }
            }
            stage('Package x86_64') {
              steps {
                sh 'build/linux.sh x86_64-unknown-linux-gnu'
              }
            }
            stage('Build Raspberry Pi') {
              steps {
                sh 'rm -rf ~/.cargo/registry/*'
                sh 'cargo clean'
                sh 'cargo build --bin psistats --release --verbose --target armv7-unknown-linux-gnueabihf'
              }
            }
            stage('Package Raspberry Pi')  {
              steps {
                sh 'build/linux.sh armv7-unknown-linux-gnueabihf'
              }
            }
          }
        }

        stage('Windows') {
          stages {
            stage('Start Build') {
              steps {
                withCredentials([string(credentialsId: 'appveyor-token', variable: 'TOKEN')]) {
                  script {
                    def appveyorBuild = appveyor_start_build(TOKEN, 'alex-dow', 'psistats-rs', env.GIT_BRANCH, env.GIT_COMMIT);
                    env.APPVEYOR_BUILD_VERSION = appveyorBuild.version;
                  }
                }
              }
            }

            stage('Wait for Appveyor') {
              steps {
                withCredentials([string(credentialsId: 'appveyor-token', variable: 'TOKEN')]) {
                  script {
                    def appveyorFinished = false;

                    def buildStatus = ""

                    while (appveyorFinished == false) {
                      buildStatus = appveyor_build_status(TOKEN, 'alex-dow', 'psistats-rs', env.APPVEYOR_BUILD_VERSION);
                      if (buildStatus == "success" || buildStatus == "error" || buildStatus == "failed" || buildStatus == 'cancelled') {
                        echo "[APPVEYOR] Finished. Result is ${buildStatus} ";
                        appveyorFinished = true;
                      } else {
                        echo "[APPVEYOR] Build status is ${buildStatus}";
                        sleep(30);
                      }
                    }

                    if (buildStatus != "success") {
                      error("Appveyor failed to build! Version: ${env.APPVEYOR_BUILD_VERSION} - Status: ${buildStatus}")
                    }
                  }
                }
              }
            }

            stage('Download Appveyor Artifacts') {
              steps {
                script {
                  appveyor_download_artifacts('alex-dow', 'psistats-rs', env.APPVEYOR_BUILD_VERSION);
                }
              }
            }
          }
        }
      }
    }
    stage('Publish') {
      steps {
        archiveArtifacts artifacts: 'target/release/psistats', onlyIfSuccessful: true
        archiveArtifacts artifacts: 'target/debian/*.deb', onlyIfSuccessful: true
        archiveArtifacts artifacts: 'target/*.msi', onlyIfSuccessful: true
      }
    }
  }
  post {
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