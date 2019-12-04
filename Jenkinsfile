def download_appveyor_artifacts(build_version, accountName, projectSlug) {

  debug('[APPVEYOR] Downloading artifacts');

  def content = httpRequest(
    url: "https://ci.appveyor.com/api/projects/${accountName}/${projectSlug}/build/${build_version}",
    customHeaders: [
      [name: 'Accept', value: 'application/json']
    ]
  );
  debug(groovy.json.JsonOutput.prettyPrint(content.getContent()));
  def build_obj = new groovy.json.JsonSlurperClassic().parseText(content.getContent());

  def job_id = build_obj.build.jobs[0].jobId;

  def artifact_response = httpRequest(
    url: "https://ci.appveyor.com/api/buildjobs/${job_id}/artifacts",
    customHeaders: [
      [name: 'Accept', value: 'application/json']
    ]
  );

  def artifact_response_content = artifact_response.getContent();
  debug(artifact_response_content);

  build_obj = new groovy.json.JsonSlurperClassic().parseText(artifact_response_content);

  build_obj.each {
    debug("[APPVEYOR] Artifact found: ${it.fileName}");
  };


}

def run_appveyor(appveyor_token, accountName, projectSlug, branch, commitId) {
    debug('[APPVEYOR] Starting')

    def request = [:]
    request['accountName'] = accountName
    request['projectSlug'] = projectSlug
    request['environmentVariables'] = env.environment;

    if (branch.startsWith('PR')) {
        debug('Building a pull request')
        def pr = branch.split('-')[1]
        request['pullRequestId'] = pr
    } else {
        debug("Building: ${branch} : ${commitId}")
        request['branch'] = branch
        request['commitId'] = commitId
    }

    def request_body = new groovy.json.JsonBuilder(request).toPrettyString();
    debug("[APPVEYOR] Request body: ${request_body}")

    def build_response = httpRequest(
        url: 'https://ci.appveyor.com/api/builds',
        httpMode: 'POST',
        customHeaders: [
            [name: 'Authorization', value: "Bearer ${appveyor_token}"],
            [name: 'Content-type', value: 'application/json']
        ],
        requestBody: request_body
    )

    def content = build_response.getContent();
    debug(groovy.json.JsonOutput.prettyPrint(content));


    def build_obj = new groovy.json.JsonSlurperClassic().parseText(content)

    debug("[APPVEYOR] Build ID: ${build_obj.buildId}");
    debug("[APPVEYOR] Build Version: ${build_obj.version}");

    def appveyor_status = 'n/a';
    def appveyor_finished = false;


    while (appveyor_finished != true) {
        def status_response = httpRequest(
            url: "https://ci.appveyor.com/api/projects/${accountName}/${projectSlug}/build/${build_obj.version}",
            httpMode: 'GET',
            customHeaders: [
                [name: 'Authorization', value: "Bearer ${appveyor_token}"],
                [name: 'Accept', value: 'application/json']
            ]
        )

        def status_content = status_response.getContent()
        debug(groovy.json.JsonOutput.prettyPrint(status_content));
        def build_data = new groovy.json.JsonSlurperClassic().parseText(status_content)

        if (build_data.build.status == "queued" || build_data.build.status == "running") {
          debug("[APPVEYOR] Waiting ... ");
          sleep(5);
        } else {
          appveyor_finished = true;
          appveyor_status   = build_data.build.status;
        }
    }

    debug("[APPVEYOR] Build completed - status: ${appveyor_status}")

    if (appveyor_status != "success") {
        error("Appveyor build failed.")
    }

    return build_obj.version;
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
        sh 'cargo clean'
        sh 'cargo install cargo-deb || true'
        sh 'cargo install cargo-rpm || true'
        sh 'cargo install cargo-config || true'
      }
    }
    stage('Build') {
      parallel {
        stage('Linux') {
          steps {
            sh 'cargo build --bin psistats --release --verbose'
          }
        }
        stage('Windows') {
          steps {
            withCredentials([string(credentialsId: APPVEYOR_TOKEN, variable: 'TOKEN')]) {
              script {
                def build_version = run_appveyor(TOKEN, 'alex-dow', 'psistats-rs', scmVars.GIT_BRANCH, scmVars.GIT_COMMIT)
                download_appveyor_artifacts(build_version, 'alex-dow', 'psistats-rs');
              }
            }
          }
        }
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