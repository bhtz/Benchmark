<?php
use Psr\Http\Message\ResponseInterface as Response;
use Psr\Http\Message\ServerRequestInterface as Request;
use Slim\Factory\AppFactory;
use Ramsey\Uuid\Uuid;

require __DIR__ . '/../vendor/autoload.php';

$app = AppFactory::create();

$app->get('/', function (Request $request, Response $response) {
    $data = array('name' => 'Benchmark.PHP');
    $r = $response->withHeader('Content-type', 'application/json');
    $r->getBody()->write(json_encode($data));
    return $r;
});

$app->get('/api/user', function (Request $request, Response $response) {
    $data = array('id' => $uuid = Uuid::uuid4(), 'firstname' => 'Benjamin', 'lastname' => 'HEINTZ', 'email' => 'heintz.benjamin@gmail.com');
    $r = $response->withHeader('Content-type', 'application/json');
    $r->getBody()->write(json_encode($data));
    return $r;
});

$app->run();
